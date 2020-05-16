use gtmserv::get_projects;
use git2::Commit;
use git2::Error;
use git2::Repository;
use gtmserv::parse_commit_note;
use gtmserv::read_projects;


#[test]
fn test_get_projects() {
  let ps = read_projects("tests/cases/project.json").unwrap();
  let ps = get_projects(&ps);
  assert_eq!(ps.len(), 10);
  assert!(ps.contains(&&"/Volumes/Data/work/#archive/emacs.d".to_string()));
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
