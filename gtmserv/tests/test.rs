use git2::Commit;
use git2::Error;
use git2::Repository;
use gtmserv::parse_commit_note;

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
