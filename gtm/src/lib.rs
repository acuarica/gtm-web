use git2::{Error, Repository};
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::Cursor;

#[macro_use]
extern crate lazy_static;

pub const GTM_REFS: &str = "refs/notes/gtm-data";

type Seconds = u32;

type Filepath = String;

type InitProjects = HashMap<String, String>;

#[derive(PartialEq, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileNote {
    pub source_file: Filepath,
    pub time_spent: Seconds,
    pub timeline: HashMap<Filepath, Seconds>,
    pub status: String,
}

#[derive(PartialEq, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommitNote {
    pub version: u32,
    pub total: Seconds,
    pub files: Vec<FileNote>,
}

#[derive(PartialEq, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Commit {
    pub author: String,
    pub date: String,
    pub when: String,
    pub hash: String,
    pub subject: String,
    pub message: String,
    pub project: String,
    pub note: CommitNote,
}

impl Commit {
    pub fn new(commit: &git2::Commit, project: &str, note: CommitNote) -> Commit {
        Commit {
            author: commit.author().name().unwrap().to_string(),
            date: "hello date".to_string(),
            when: "hello when".to_string(),
            hash: commit.id().to_string(),
            subject: commit.summary().unwrap().to_string(),
            message: commit.message().unwrap().to_string(),
            project: project.to_string(),
            note,
        }
    }
}

pub fn read_projects(filename: &str) -> Result<InitProjects, std::io::Error> {
    let file = File::open(filename)?;
    let ps = serde_json::from_reader(file)?;
    Ok(ps)
}

pub fn get_projects(init_projects: &InitProjects) -> Vec<&String> {
    let mut result = Vec::new();
    for k in init_projects.keys() {
        result.push(k);
    }

    result
}

pub fn parse_file_entry(file_entry: &str) -> Result<FileNote, &str> {
    let parts: Vec<&str> = file_entry.split(',').collect();
    if parts.len() < 3 {
        return Err("Invalid file entry, not enough entries");
    }
    let file_name: Vec<&str> = parts[0].split(':').collect();
    if file_name.len() < 2 {
        return Err("Invalid file entry format");
    }

    let mut timeline = HashMap::new();
    for i in 1..parts.len() - 1 {
        let timeline_entry: Vec<&str> = parts[i].split(':').collect();
        if timeline_entry.len() < 2 {
            return Err("Invalid timeline entry format");
        }
        timeline.insert(
            timeline_entry[0].to_string(),
            match timeline_entry[1].parse::<Seconds>() {
                Err(_) => return Err("Invalid spent time in timeline entry"),
                Ok(value) => value,
            },
        );
    }

    let note = FileNote {
        source_file: file_name[0].to_string(),
        time_spent: match file_name[1].parse::<Seconds>() {
            Err(_) => return Err("Invalid total spent time"),
            Ok(value) => value,
        },
        timeline,
        status: match *parts.last().unwrap() {
            s @ "m" | s @ "r" | s @ "d" => s.to_string(),
            _ => return Err("Invalid status"),
        },
    };
    Ok(note)
}

pub fn parse_commit_note(message: &str) -> Result<CommitNote, &str> {
    lazy_static! {
        static ref VERSION_RE: Regex = Regex::new(r"\[ver:(\d+),total:(\d+)\]").unwrap();
    }
    let mut lines = Cursor::new(message).lines();
    let mut commit = match lines.next() {
        None => return Err("No version found"),
        Some(Err(_)) => return Err("Error found"),
        Some(Ok(first)) => match VERSION_RE.captures_iter(&first.to_string()).next() {
            None => return Err("Invalid version header"),
            Some(parts) => CommitNote {
                version: match parts[1].parse::<Seconds>() {
                    Err(_) => return Err("Invalid version"),
                    Ok(value) => value,
                },
                total: match parts[2].parse::<Seconds>() {
                    Err(_) => return Err("Invalid total"),
                    Ok(value) => value,
                },
                files: Vec::new(),
            },
        },
    };
    for line in lines {
        match parse_file_entry(&line.unwrap()) {
            Err(_) => return Err("invalid file"),
            Ok(entry) => commit.files.push(entry),
        };
    }
    Ok(commit)
}

pub fn get_commits(_path: &str) -> Result<(), Error> {
    // let repo = Repository::open(path)?;
    // let nt = repo.notes(Some("refs/notes/gtm-data")).unwrap();
    Ok(())
}

pub fn get_notes(repo: &Repository) -> Result<Vec<Commit>, Error> {
    let notes = repo.notes(Some(GTM_REFS))?;
    let project = repo.path().to_str().unwrap();

    let mut result = Vec::new();
    for note in notes {
        let p = note.unwrap();
        let note = repo.find_note(Some(GTM_REFS), p.1)?;
        if let Some(message) = note.message() {
            if let Ok(commit_note) = parse_commit_note(message) {
                let commit = repo.find_commit(p.1)?;
                result.push(Commit::new(&commit, project, commit_note));
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

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
            FileNote {
                source_file: "demo/demo.ts".to_string(),
                time_spent: 60,
                timeline: [("1585918800".to_string(), 60)].iter().cloned().collect(),
                status: "r".to_string(),
            }
        );
    }
}
