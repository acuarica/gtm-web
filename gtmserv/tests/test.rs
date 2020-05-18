#![feature(int_error_matching)]

use git2::Commit;
use git2::Error;
use git2::Repository;
use gtmserv::get_projects;
use gtmserv::get_status;
use gtmserv::parse_commit_note;
use gtmserv::read_projects;
use gtmserv::FileEvent;

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

#[test]
fn status() {
    let swd = vec![
        FileEvent::new(1589673491, "src/file1.ts"),
        FileEvent::new(1589673494, "src/file2.ts"),
        FileEvent::new(1589673601, "test/test1.ts"),
        FileEvent::new(1589673632, "test/test2.ts"),
        FileEvent::new(1589673658, "assets/logo.png"),
        FileEvent::new(1589673732, "assets/main.css"),
    ];

    let map = get_status(swd);

    let bin = map.get(&1589673480).unwrap();
    assert_eq!(bin.timespent("src/file1.ts".to_string()), 30);
    assert_eq!(bin.timespent("src/file2.ts".to_string()), 30);

    let bin = map.get(&1589673600).unwrap();
    assert_eq!(bin.timespent("test/test1.ts".to_string()), 20);
    assert_eq!(bin.timespent("test/test2.ts".to_string()), 20);
    assert_eq!(bin.timespent("assets/logo.png".to_string()), 20);

    let bin = map.get(&1589673720).unwrap();
    assert_eq!(bin.timespent("assets/main.css".to_string()), 60);

    assert_eq!(map.commit_note().total, 180);
}
