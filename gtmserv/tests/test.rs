#![feature(int_error_matching)]

use git2::{Commit, Error, Repository};
use gtmserv::{get_projects, parse_commit_note, read_projects};
use std::io::{self, Write};
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

#[test]
fn test() {

    // TempDir
    // let mut file = NamedTempFile::new()?;
    // env::set_var("HOME", "yes");
}

pub fn create_test_repo() -> Result<(), Error> {
    // let repo = Repository::open("/Users/luigi/work/home")?;
    // let odb = repo.odb()?;

    Ok(())
}

// #[test]
pub fn test_commits() -> Result<(), Error> {
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
