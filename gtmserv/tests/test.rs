#![feature(int_error_matching)]

mod init_projects_tests {

    use gtmserv::InitProjects;
    use io::Write;
    use std::io;
    use tempfile::NamedTempFile;

    pub(crate) const PROJECT_JSON: &[u8] =
        br#"{"/path/to/emacs.d":"2020-05-04T04:39:54.911709457+02:00",
            "/path/to/codemirror.next":"2020-05-04T04:38:18.093292086+02:00",
            "/path/to/gtm":"2020-05-04T04:35:28.761863254+02:00",
            "/path/to/gtm/web":"2020-05-04T04:44:39.112956448+02:00"}"#;

    fn assert_error<T>(result: Result<T, io::Error>, code: Option<i32>, kind: io::ErrorKind) {
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.raw_os_error(), code, "Got err: {:?}", err);
        assert_eq!(err.kind(), kind, "Got err: {:?}", err);
    }

    #[test]
    #[cfg_attr(target_os="windows", ignore)]
    fn init_projects_with_non_existing_file() {
        assert_error(
            InitProjects::from_file("/non/existing/path"),
            Some(2),
            io::ErrorKind::NotFound,
        );
    }

    #[test]
    fn init_projects_with_non_json_file() -> Result<(), io::Error> {
        let mut file = NamedTempFile::new()?;
        file.write(b"Non valid JSON")?;
        assert_error(
            InitProjects::from_file(file.path()),
            None,
            io::ErrorKind::InvalidData,
        );
        Ok(())
    }

    #[test]
    fn init_projects_from_json_file() -> Result<(), io::Error> {
        let mut file = NamedTempFile::new()?;
        file.write(PROJECT_JSON)?;
        let ps = InitProjects::from_file(file.path()).unwrap();
        assert_eq!(ps.len(), 4);
        assert!(ps.contains_project("/path/to/gtm"));
        Ok(())
    }

    #[test]
    fn init_projects_from_empty_json_file() -> Result<(), io::Error> {
        let mut file = NamedTempFile::new()?;
        file.write(b"{}")?;
        assert_eq!(InitProjects::from_file(file.path()).unwrap().len(), 0);
        Ok(())
    }

    #[test]
    fn get_init_project_list_from_json_file() -> Result<(), io::Error> {
        let mut file = NamedTempFile::new()?;
        file.write(PROJECT_JSON)?;
        let ps = InitProjects::from_file(file.path()).unwrap();
        let ps = ps.get_project_list();
        assert_eq!(ps.len(), 4);
        assert_eq!(
            {
                let mut v = ps.into_iter().collect::<Vec<&String>>();
                v.sort();
                v
            },
            vec![
                "/path/to/codemirror.next",
                "/path/to/emacs.d",
                "/path/to/gtm",
                "/path/to/gtm/web"
            ]
        );
        Ok(())
    }
}

mod notes_tests {

    use git2::Oid;
    use git2::{Repository, Signature};
    use gtmserv::get_notes;
    use gtmserv::parse_commit_note;
    use gtmserv::GTM_REFS;
    use std::error::Error;
    use tempfile::tempdir;

    #[test]
    fn test_notes() -> Result<(), Box<dyn Error>> {
        let temp = tempdir()?;
        let repo_path = temp.path();
        println!("Using repo path: {:?}", repo_path);

        let repo = Repository::init(repo_path)?;
        let sig = Signature::now("My name is test", "test@test.io")?;
        let oid = create_commit(&repo, &sig)?;

        repo.note(&sig, &sig, Some(GTM_REFS), oid, "[ver:1,total:180]", false)?;

        let mut cs = Vec::new();
        get_notes(&mut cs, &repo, "test".to_owned(), 0, 2589945042, &None)?;
        println!("{:?}", cs);

        Ok(())
    }

    fn create_commit(repo: &Repository, sig: &Signature) -> Result<Oid, git2::Error> {
        let tree_id = {
            let mut index = repo.index()?;
            index.write_tree()?
        };

        let tree = repo.find_tree(tree_id)?;
        repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
    }

    // #[test]
    pub fn _test_commits() -> Result<(), git2::Error> {
        let repo = Repository::open("tests/cases/repo")?;
        let mut revwalk = repo.revwalk()?;

        revwalk.push_head()?;

        for oid in revwalk {
            let oid = oid?;
            let commit = repo.find_commit(oid)?;
            println!("{}", commit.author());
            println!("{}", commit.message().unwrap());

            let note = repo.find_note(Some("refs/notes/gtm-data"), oid);
            // println!("note: {:?}", p);
            if note.is_ok() {
                let note = note.unwrap();
                let msg = note.message().unwrap();
                let cn = parse_commit_note(msg);
                println!("{}, {:?}", msg, cn);
            }
        }

        Ok(())
    }
}
mod cli_tests {

    const GTM_CMD: &str = "gtmserv";

    use crate::init_projects_tests;
    use crate::init_projects_tests::PROJECT_JSON;
    use assert_cmd::Command;
    use gtmserv::Commit;
    use predicates::prelude::*;
    use std::collections::HashMap;
    use std::error::Error;
    use std::fs;
    use tempfile::{tempdir, TempDir};

    fn create_config_file(json_text: &[u8]) -> Result<TempDir, Box<dyn Error>> {
        print!("Using temp dir as HOME ... ");
        let home = tempdir()?;
        println!("{:?} [OK]", &home.path());
        let mut path = home.path().to_path_buf();
        path.push(".git-time-metric");
        fs::create_dir(&path)?;
        path.push("project.json");
        print!("Writing {:?} ... ", &path);
        fs::write(&path, json_text)?;
        println!("[OK]");
        Ok(home)
    }

    #[test]
    #[cfg_attr(target_os="windows", ignore)]
    fn run_projects_from_env_with_empty_json() -> Result<(), Box<dyn Error>> {
        let home = create_config_file(b"{}")?;
        Command::cargo_bin(GTM_CMD)?
            .env("HOME", home.path())
            .arg("projects")
            .assert()
            .success()
            .stdout(predicate::function(|out| {
                let list: Vec<String> = serde_json::from_slice(out).unwrap();
                println!("Got from stdout: {:?}", list);
                list.len() == 0
            }))
            .stderr(predicate::str::is_empty());
        Ok(())
    }

    #[test]
    #[cfg_attr(target_os="windows", ignore)]
    fn run_projects_from_env_with_json() -> Result<(), Box<dyn Error>> {
        let home = create_config_file(init_projects_tests::PROJECT_JSON)?;
        Command::cargo_bin(GTM_CMD)?
            .env("HOME", home.path())
            .arg("projects")
            .assert()
            .success()
            .stdout(predicate::function(|out| {
                let list: Vec<String> = serde_json::from_slice(out).unwrap();
                let map: HashMap<String, String> = serde_json::from_slice(PROJECT_JSON).unwrap();
                list.len() == map.len() && map.keys().all(|p| list.contains(p))
            }))
            .stderr(predicate::str::is_empty());
        Ok(())
    }

    #[test]
    fn run_projects_from_env_with_no_json() -> Result<(), Box<dyn Error>> {
        Command::cargo_bin(GTM_CMD)?
            .env("HOME", "/non/existing/path")
            .arg("projects")
            .assert()
            .failure()
            .stdout(predicate::str::is_empty());
        Ok(())
    }

    #[test]
    fn run_projects_from_env_with_invalid_json() -> Result<(), Box<dyn Error>> {
        let home = create_config_file(b"No JSON data here")?;
        Command::cargo_bin(GTM_CMD)?
            .env("HOME", home.path())
            .arg("projects")
            .assert()
            .failure()
            .stdout(predicate::str::is_empty());
        Ok(())
    }

    #[test]
    #[cfg_attr(target_os="windows", ignore)]
    fn run_commits_no_args() -> Result<(), Box<dyn std::error::Error>> {
        let home = create_config_file(b"{}")?;
        Command::cargo_bin(GTM_CMD)?
            .env("HOME", home.path())
            .arg("commits")
            .assert()
            .success()
            .stdout(predicate::function(|out| {
                let result: Vec<Commit> = serde_json::from_slice(out).unwrap();
                result.len() == 0
            }))
            .stderr(predicate::str::is_empty());
        Ok(())
    }
}
