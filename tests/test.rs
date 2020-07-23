#![feature(int_error_matching)]
#![feature(test)]

extern crate test;

#[macro_use]
extern crate maplit;

mod init_projects_tests {

    use gtm::projects::Projects;
    use io::Write;
    use std::{
        io,
        path::{Path, PathBuf},
    };
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
    #[cfg_attr(target_os = "windows", ignore)]
    fn init_projects_with_non_existing_file() {
        assert_error(
            Projects::from_file("/non/existing/path"),
            Some(2),
            io::ErrorKind::NotFound,
        );
    }

    #[test]
    fn init_projects_with_non_json_file() -> Result<(), io::Error> {
        let mut file = NamedTempFile::new()?;
        file.write(b"Non valid JSON")?;
        assert_error(
            Projects::from_file(file.path()),
            None,
            io::ErrorKind::InvalidData,
        );
        Ok(())
    }

    #[test]
    fn init_projects_from_json_file() -> Result<(), io::Error> {
        let mut file = NamedTempFile::new()?;
        file.write(PROJECT_JSON)?;
        let ps = Projects::from_file(file.path()).unwrap();
        assert_eq!(ps.len(), 4);
        assert!(ps.contains_project("/path/to/gtm"));
        Ok(())
    }

    #[test]
    fn init_projects_from_empty_json_file() -> Result<(), io::Error> {
        let mut file = NamedTempFile::new()?;
        file.write(b"{}")?;
        assert_eq!(Projects::from_file(file.path()).unwrap().len(), 0);
        Ok(())
    }

    #[test]
    fn get_init_project_list_from_json_file() -> Result<(), io::Error> {
        let mut file = NamedTempFile::new()?;
        file.write(PROJECT_JSON)?;
        let ps = Projects::from_file(file.path()).unwrap();
        let ps = ps.keys();
        assert_eq!(ps.len(), 4);
        assert_eq!(
            {
                let mut v = ps.into_iter().collect::<Vec<&PathBuf>>();
                v.sort();
                v
            },
            vec![
                "/path/to/codemirror.next",
                "/path/to/emacs.d",
                "/path/to/gtm",
                "/path/to/gtm/web"
            ]
            .iter()
            .map(Path::new)
            .collect::<Vec<&Path>>()
        );
        Ok(())
    }
}

mod notes_tests {

    use git2::{Oid, Repository, Signature};
    use gtm::{
        get_notes, parse::parse_commit_note, Commit, CommitNote, FileNote, NotesFilter, GTM_REFS,
    };
    use std::error::Error;
    use tempfile::{tempdir, TempDir};

    struct TempRepo<'repo> {
        _tempdir: TempDir,
        repo: Repository,
        sig: Signature<'repo>,
    }

    impl<'repo> TempRepo<'repo> {
        fn new() -> Result<TempRepo<'repo>, Box<dyn Error>> {
            let tempdir = tempdir()?;
            let repo_path = tempdir.path();
            println!("Using repo path: {:?}", repo_path);

            let repo = Repository::init(repo_path)?;
            let sig = Signature::now("Test Repo", "test@repo.io")?;
            Ok(TempRepo {
                _tempdir: tempdir,
                repo,
                sig,
            })
        }

        fn commit(self: &mut Self, message: &str) -> Result<TempOid, git2::Error> {
            let tree_id = {
                let mut index = self.repo.index()?;
                index.write_tree()?
            };

            let tree = self.repo.find_tree(tree_id)?;
            let parent: Vec<git2::Commit> = match self.repo.head() {
                Ok(r) => vec![r.peel_to_commit()?],
                Err(_) => vec![],
            };
            let parent: Vec<&git2::Commit> = parent.iter().map(|e| e).collect();

            Ok(TempOid(
                self.repo.commit(
                    Some("HEAD"),
                    &self.sig,
                    &self.sig,
                    message,
                    &tree,
                    parent.as_slice(),
                )?,
                &&self.repo,
                &self.sig,
            ))
        }
    }

    struct TempOid<'repo>(Oid, &'repo Repository, &'repo Signature<'repo>);

    impl<'repo> TempOid<'repo> {
        fn note(&self, note: &str) -> Result<Oid, git2::Error> {
            self.1
                .note(&self.2, &self.2, Some(GTM_REFS), self.0, note, false)
        }

        fn as_commit(&self) -> Result<git2::Commit<'repo>, git2::Error> {
            self.1.find_commit(self.0)
        }
    }

    #[test]
    fn test_no_notes_commit() -> Result<(), Box<dyn Error>> {
        let mut repo = TempRepo::new()?;
        for _ in 0..100 {
            repo.commit("asdf")?;
        }

        assert!(get_notes(
            |_| (),
            &repo.repo,
            "test",
            &NotesFilter::all()
        )
        .is_err());
        Ok(())
    }

    #[test]
    fn test_no_files_commit_note() -> Result<(), Box<dyn Error>> {
        let mut repo = TempRepo::new()?;
        repo.commit("Message")?.note("[ver:1,total:0]")?;

        let mut cs = Vec::new();
        get_notes(
            |cn| cs.push(cn),
            &repo.repo,
            "test",
            &NotesFilter::all(),
        )?;
        assert_eq!(cs.len(), 1);
        Ok(())
    }

    #[test]
    fn test_notes() -> Result<(), Box<dyn Error>> {
        let mut repo = TempRepo::new()?;
        for _ in 0..10 {
            repo.commit("Message")?.note(
                "[ver:2,total:213]
closebrackets/src/closebrackets.ts:950,1585918800:510,1585922400:400,1585929600:40,r
text/src/char.ts:90,1585918800:90,r",
            )?;
        }

        let mut commits = Vec::new();
        get_notes(
            |cn| {
                commits.push(cn);
            },
            &repo.repo,
            "test",
            &NotesFilter::all(),
        )?;
        assert_eq!(commits.len(), 10);
        for c in commits {
            assert_eq!(
                c.commit.note,
                CommitNote {
                    version: 2,
                    total: 213,
                    files: vec![
                        FileNote {
                            source_file: "closebrackets/src/closebrackets.ts",
                            time_spent: 950,
                            timeline: btreemap! {
                                1585918800 => 510,
                                1585922400 => 400,
                                1585929600 => 40,
                            },
                            status: "r",
                        },
                        FileNote {
                            source_file: "text/src/char.ts",
                            time_spent: 90,
                            timeline: btreemap! { 1585918800 => 90, },
                            status: "r",
                        }
                    ],
                }
            );
        }

        Ok(())
    }

    #[test]
    fn test_commit_message() -> Result<(), Box<dyn Error>> {
        let mut repo = TempRepo::new()?;
        let commit = repo
            .commit("Commit message subject\n\nCommit message body.")?
            .as_commit()?;
        let commit = Commit::new(&commit, "asdf".to_owned(), CommitNote::new(1, 0));
        assert_eq!(commit.subject, "Commit message subject");
        assert_eq!(commit.message, "Commit message body.");

        Ok(())
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

    const GTM_CMD: &str = "gtmcli";

    use crate::init_projects_tests;
    use crate::init_projects_tests::PROJECT_JSON;
    use assert_cmd::Command;
    use gtm::Commit;
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
    #[cfg_attr(target_os = "windows", ignore)]
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
    #[cfg_attr(target_os = "windows", ignore)]
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
    #[cfg_attr(target_os = "windows", ignore)]
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
