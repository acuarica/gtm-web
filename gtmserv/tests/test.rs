use gtmserv::get_status;
use git2::Commit;
use git2::Error;
use git2::Repository;
use gtmserv::parse_commit_note;
use gtmserv::FileEvent;

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
        FileEvent::new(1589673662, "dist/dist1/ts"),
        FileEvent::new(1589673678, "assets/logo.png"),
        FileEvent::new(1589673732, "assets/main.css"),
    ];

    let map = get_status(swd);
    assert!(map.get(&1589673480).unwrap().contains_key("src/file1.ts"));
}
