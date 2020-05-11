use gtm::parse_commit_note;
use gtm::parse_file_entry;
use gtm::read_projects;
use gtm::CommitNote;
use gtm::FileNote;

#[test]
fn test_projects() {
  let ps = read_projects("tests/cases/project.json").unwrap();
  assert_eq!(ps.len(), 10);
}

#[test]
fn test_empty_projects() {
  let ps = read_projects("tests/cases/project-empty.json").unwrap();
  assert_eq!(ps.len(), 0);
}

#[test]
fn test_parse_file_entry_invalid() {
  assert!(parse_file_entry("").is_err());
  assert!(parse_file_entry("src/file.ts:2797").is_err());
  assert!(parse_file_entry("src/file.ts:2797,m").is_err());
  assert!(parse_file_entry("src/file.ts2797,1585861200:354,m").is_err());
  assert!(parse_file_entry("src/file.ts:123abc,1585861200:354,m").is_err());
  assert!(parse_file_entry("src/file.ts:123,1585861200:354,a").is_err());
}

#[test]
fn test_parse_file_entry() {
  assert_eq!(
    parse_file_entry("src/file.ts:150,1585861200:60,1585875600:90,m").unwrap(),
    FileNote {
      source_file: "src/file.ts".to_string(),
      time_spent: 150,
      timeline: [
        ("1585861200".to_string(), 60),
        ("1585875600".to_string(), 90)
      ]
      .iter()
      .cloned()
      .collect(),
      status: "m".to_string(),
    }
  );

  assert_eq!(
    parse_file_entry("comment/src/comment.ts:2797,1585861200:354,1585875600:50,1585879200:240,1585908000:444,1585918800:1629,1585929600:80,m")
      .unwrap(),
    FileNote {
      source_file: "comment/src/comment.ts".to_string(),
      time_spent: 2797,
      timeline: [
        ("1585861200".to_string(), 354),
        ("1585875600".to_string(), 50),
        ("1585879200".to_string(), 240),
        ("1585908000".to_string(), 444),
        ("1585918800".to_string(), 1629),
        ("1585929600".to_string(), 80),
      ]
      .iter()
      .cloned()
      .collect(),
      status: "m".to_string(),
    }
  );
}

#[test]
fn test_commit_note_invalid() {
  assert!(parse_commit_note("").is_err());
  assert!(parse_commit_note("[]").is_err());
  assert!(parse_commit_note("[ver:1total:213]").is_err());
  assert!(parse_commit_note("[ver:a,total:213]").is_err());
  assert!(parse_commit_note("[ver:1,total:a]").is_err());
}

#[test]
fn test_commit_note() {
  assert_eq!(
    parse_commit_note("[ver:2,total:213]").unwrap(),
    CommitNote {
      version: 2,
      total: 213,
      files: Vec::new(),
    }
  );

  assert_eq!(
    parse_commit_note(
      "[ver:2,total:213]
closebrackets/src/closebrackets.ts:950,1585918800:510,1585922400:400,1585929600:40,r
text/src/char.ts:90,1585918800:90,r"
    )
    .unwrap(),
    CommitNote {
      version: 2,
      total: 213,
      files: vec![
        FileNote {
          source_file: "closebrackets/src/closebrackets.ts".to_string(),
          time_spent: 950,
          timeline: [
            ("1585918800".to_string(), 510),
            ("1585922400".to_string(), 400),
            ("1585929600".to_string(), 40),
          ]
          .iter()
          .cloned()
          .collect(),
          status: "r".to_string(),
        },
        FileNote {
          source_file: "text/src/char.ts".to_string(),
          time_spent: 90,
          timeline: [("1585918800".to_string(), 90),].iter().cloned().collect(),
          status: "r".to_string(),
        }
      ],
    }
  );

  let note = parse_commit_note("[ver:1,total:4037]
comment/src/comment.ts:2797,1585861200:354,1585875600:50,1585879200:240,1585908000:444,1585918800:1629,1585929600:80,m
closebrackets/src/closebrackets.ts:950,1585918800:510,1585922400:400,1585929600:40,r
text/src/char.ts:90,1585918800:90,r
demo/demo.ts:60,1585918800:60,r
state/src/selection.ts:40,1585918800:40,r
highlight/src/highlight.ts:30,1585918800:30,r
lang-javascript/src/javascript.ts:30,1585918800:30,r
node_modules/w3c-keyname/index.d.ts:20,1585922400:20,r
CHANGELOG.md:20,1585918800:20,r").unwrap();

  assert_eq!(note.version, 1);
  assert_eq!(note.total, 4037);
  assert_eq!(note.files.len(), 9);
  assert_eq!(
    note.files[3],
    gtm::FileNote {
      source_file: "demo/demo.ts".to_string(),
      time_spent: 60,
      timeline: [("1585918800".to_string(), 60)].iter().cloned().collect(),
      status: "r".to_string(),
    }
  );
}

#[test]
fn test_commits() {
  let c = gtm::commits();
  assert!(c.is_ok());
}
