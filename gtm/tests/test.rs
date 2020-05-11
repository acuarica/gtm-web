#[test]
fn test_projects() {
  let ps = gtm::read_projects("tests/cases/project.json").unwrap();
  assert_eq!(ps.len(), 10);
}

#[test]
fn test_empty_projects() {
  let ps = gtm::read_projects("tests/cases/project-empty.json").unwrap();
  assert_eq!(ps.len(), 0);
}

#[test]
fn test_parse_file_entry_invalid() {
  assert!(gtm::parse_file_entry("").is_err());
  assert!(gtm::parse_file_entry("src/file.ts:2797").is_err());
  assert!(gtm::parse_file_entry("src/file.ts:2797,m").is_err());
  assert!(gtm::parse_file_entry("src/file.ts2797,1585861200:354,m").is_err());
  assert!(gtm::parse_file_entry("src/file.ts:123abc,1585861200:354,m").is_err());
  assert!(gtm::parse_file_entry("src/file.ts:123,1585861200:354,a").is_err());
}

#[test]
fn test_parse_file_entry() {
  let entry = "src/file.ts:150,1585861200:60,1585875600:90,m";
  let note = gtm::parse_file_entry(entry).unwrap();
  assert_eq!(note.source_file, "src/file.ts");
  assert_eq!(note.status, "m");
  assert_eq!(note.timeline.len(), 2);
  assert_eq!(*note.timeline.get("1585861200").unwrap(), 60);
  assert_eq!(*note.timeline.get("1585875600").unwrap(), 90);
}

#[test]
fn test_parse_file_entry_full() {
  let entry = "comment/src/comment.ts:2797,1585861200:354,1585875600:50,1585879200:240,1585908000:444,1585918800:1629,1585929600:80,m";
  let note = gtm::parse_file_entry(entry).unwrap();

  assert_eq!(note.source_file, "comment/src/comment.ts");
}

#[test]
fn test_note() {
  let note = " [ver:1,total:4037]
comment/src/comment.ts:2797,1585861200:354,1585875600:50,1585879200:240,1585908000:444,1585918800:1629,1585929600:80,m
closebrackets/src/closebrackets.ts:950,1585918800:510,1585922400:400,1585929600:40,r
text/src/char.ts:90,1585918800:90,r
demo/demo.ts:60,1585918800:60,r
state/src/selection.ts:40,1585918800:40,r
highlight/src/highlight.ts:30,1585918800:30,r
lang-javascript/src/javascript.ts:30,1585918800:30,r
node_modules/w3c-keyname/index.d.ts:20,1585922400:20,r
CHANGELOG.md:20,1585918800:20,r
  ";

  gtm::notes(note);

  assert_eq!(1, 1);


  let nn = "[ver:1,total:3930]
  demo/demo.ts:1011,1585832400:315,1585836000:676,1585839600:20,m
  comment/src/index.ts:825,1585832400:213,1585836000:592,1585839600:20,m
  .gtm/terminal.app:685,1585832400:540,1585836000:145,r
  package.json:315,1585836000:308,1585839600:7,m
  matchbrackets/src/matchbrackets.ts:180,1585832400:180,r
  .gitignore:135,1585825200:60,1585832400:75,r
  comment/package.json:106,1585832400:53,1585836000:40,1585839600:13,m
  bin/cm.js:105,1585836000:105,r
  closebrackets/src/closebrackets.ts:71,1585832400:60,1585836000:11,r
  keymap/src/keymap.ts:60,1585832400:60,r
  fold/src/fold.ts:60,1585832400:50,1585836000:10,r
  bin/package.json:55,1585836000:55,r
  commands/package.json:44,1585832400:44,r
  demo/demo.js:35,1585832400:20,1585836000:15,r
  fold/package.json:30,1585832400:20,1585836000:10,r
  gutter/package.json:25,1585832400:25,r
  tsconfig.json:22,1585836000:22,r
  closebrackets/package.json:20,1585836000:20,r
  tsconfig.base.json:19,1585836000:19,r
  commands/src/README.md:15,1585832400:15,r
  commands/src/commands.ts:15,1585832400:15,r
  autocomplete/src/index.ts:15,1585832400:15,r
  bin/console/index.d.ts:15,1585836000:15,r
  README.md:12,1585836000:12,r
  gutter/src/index.ts:10,1585832400:10,r
  comment/src/index.js:10,1585836000:10,r
  closebrackets/src/closebrackets.d.ts:7,1585836000:7,r
  closebrackets/src/closebrackets.js:7,1585836000:7,r
  closebrackets/src/closebrackets.d.ts.map:7,1585836000:7,r
  closebrackets/dist/index.js:7,1585836000:7,r
  closebrackets/dist/index.js.map:7,1585836000:7,r";

  println!("{}", nn)
}
