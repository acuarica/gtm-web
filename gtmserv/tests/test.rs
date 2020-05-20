#![feature(int_error_matching)]

use git2::Oid;
use git2::{Commit, Repository, Signature};
use gtmserv::GTM_REFS;
use gtmserv::{get_notes, get_projects, parse_commit_note, read_projects};
use std::error::Error;
use std::io::{self, Write};
use tempfile::tempdir;
use tempfile::NamedTempFile;

const PROJECT_JSON: &[u8] = br#"{"/path/to/emacs.d":"2020-05-04T04:39:54.911709457+02:00",
            "/path/to/codemirror.next":"2020-05-04T04:38:18.093292086+02:00",
            "/path/to/gtm":"2020-05-04T04:35:28.761863254+02:00",
            "/path/to/gtm/web":"2020-05-04T04:44:39.112956448+02:00"}"#;

#[test]
fn read_init_projects() -> Result<(), io::Error> {
    let mut file = NamedTempFile::new()?;
    file.write(PROJECT_JSON)?;
    let ps = read_projects(file.path()).unwrap();
    assert_eq!(ps.len(), 4);
    assert!(ps.contains_key("/path/to/gtm"));
    Ok(())
}

#[test]
fn read_empty_init_projects() -> Result<(), io::Error> {
    let mut file = NamedTempFile::new()?;
    file.write(b"{}")?;
    assert_eq!(read_projects(file.path()).unwrap().len(), 0);
    Ok(())
}

#[test]
fn gets_init_projects() -> Result<(), io::Error> {
    let mut file = NamedTempFile::new()?;
    file.write(PROJECT_JSON)?;
    let ps = read_projects(file.path()).unwrap();
    let ps = get_projects(&ps);
    assert_eq!(ps.len(), 4);
    assert!(ps.contains(&&"/path/to/gtm".to_string()));
    Ok(())
}

mod projects {

    use assert_cmd::Command;

    #[test]
    fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("gtmserv")?;
        cmd.arg("projects").assert().success();
        // .stderr(predicate::str::contains("No such file or directory"));

        Ok(())
    }
}

#[test]
fn test() {

    // TempDir
    // let mut file = NamedTempFile::new()?;
    // env::set_var("HOME", "yes");
}

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
    get_notes(&mut cs, &repo, "test".to_owned(), 0, 2589945042)?;
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
pub fn test_commits() -> Result<(), git2::Error> {
    let repo = Repository::open("tests/cases/repo")?;
    let mut revwalk = repo.revwalk()?;

    revwalk.push_head()?;

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        pc(&commit);

        let note = repo.find_note(Some("refs/notes/gtm-data"), oid);
        // println!("note: {:?}", p);
        if note.is_ok() {
            let note = note.unwrap();
            let msg = note.message().unwrap();
            let cn = parse_commit_note(msg);
            println!("{}, {:?}", msg, cn);
        }
    }

    // repo.head()

    Ok(())
}

fn pc(commit: &Commit) {
    println!("{}", commit.author());
    println!("{}", commit.message().unwrap());
}
