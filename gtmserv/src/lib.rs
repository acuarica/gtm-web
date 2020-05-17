use chrono::NaiveDate;
use chrono::ParseResult;
use git2::{Error, Repository};
use regex::Regex;
use serde::Serialize;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, Cursor},
    path::Path,
};

#[macro_use]
extern crate lazy_static;

extern crate chrono;

pub const GTM_REFS: &str = "refs/notes/gtm-data";

type Seconds = u32;

type Filepath = String;

/// Represents initialized projects by `gtm`.
/// It is represented by a `HashMap` where the keys are paths and
/// the values are formatted dates.
/// The keys are the repository path of the working directory of the
/// git repository.
/// The values indicates when the git repo was `init` by gtm.
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
    pub fn new(commit: &git2::Commit, project: String, note: CommitNote) -> Commit {
        use chrono::{TimeZone, Utc};
        use git2::Time;
        let formatdate = |time: Time| Utc.timestamp(time.seconds(), 0).to_string();

        Commit {
            author: commit.author().name().unwrap().to_string(),
            date: formatdate(commit.time()),
            when: formatdate(commit.author().when()),
            hash: commit.id().to_string(),
            subject: commit.summary().unwrap().to_string(),
            message: commit.message().unwrap().to_string(),
            project,
            note,
        }
    }
}

pub fn read_projects<P: AsRef<Path>>(filename: P) -> Result<InitProjects, std::io::Error> {
    let file = File::open(filename)?;
    let ps = serde_json::from_reader(file)?;
    Ok(ps)
}

pub fn get_projects(init_projects: &InitProjects) -> Vec<String> {
    let mut result = Vec::new();
    for k in init_projects.keys() {
        result.push(k.to_owned());
    }

    result
}

pub fn fetch_projects() -> Option<Vec<String>> {
    let mut path = dirs::home_dir()?;
    path.push(".git-time-metric");
    path.push("project.json");
    let init_projects = match read_projects(path) {
        Ok(value) => value,
        Err(_err) => return None,
    };
    Some(get_projects(&init_projects))
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

pub fn to_unixtime(date: String) -> ParseResult<i64> {
    let date = NaiveDate::parse_from_str(date.as_ref(), "%Y-%m-%d")?;
    Ok(date.and_hms(0, 0, 0).timestamp())
}

pub fn get_notes(
    result: &mut Vec<Commit>,
    repo: &Repository,
    project: String,
    from_date: i64,
    to_date: i64,
) -> Result<(), Error> {
    let notes = repo.notes(Some(GTM_REFS))?;

    for note in notes {
        let p = note.unwrap();
        let note = repo.find_note(Some(GTM_REFS), p.1)?;
        if let Some(message) = note.message() {
            let commit = repo.find_commit(p.1)?;
            let time = commit.time().seconds();
            if time >= from_date && time <= to_date {
                if let Ok(commit_note) = parse_commit_note(message) {
                    result.push(Commit::new(&commit, project.to_owned(), commit_note));
                }
            }
        }
    }

    Ok(())
}

pub struct FileEvent {
    timestamp: u64,
    filename: String,
}

impl FileEvent {
    pub fn new(timestamp: u64, filename: &str) -> FileEvent {
        FileEvent {
            timestamp,
            filename: filename.to_string(),
        }
    }
}

/// Given a Unix epoch,
/// returns a Unix epoch rounded down to the minute.
/// It is used to create bins at the minute granularity.
///
/// # Examples
///
/// ```
/// assert_eq!(gtmserv::down_to_minute(1589673494), 1589673480);
/// ```
pub fn down_to_minute(timestamp: u64) -> u64 {
    (timestamp / 60) * 60
}

pub struct TimelineBin {
    filemap: HashMap<Filepath, usize>,
    count: usize,
}

impl TimelineBin {
    fn new() -> TimelineBin {
        TimelineBin {
            filemap: HashMap::new(),
            count: 0,
        }
    }

    fn append(self: &mut Self, filepath: Filepath) {
        self.count += 1;
        let count = self.filemap.entry(filepath).or_insert(0);
        *count += 1;
    }

    pub fn timespent(self: &Self, filepath: Filepath) -> Seconds {
        let count = self.filemap.get(&filepath).unwrap();
        (60 * count / self.count) as u32
    }
}

type StatusWorkdir = Vec<FileEvent>;

type FileEventMap = HashMap<u64, TimelineBin>;

/// Creates a file event map.
pub fn get_status(swd: StatusWorkdir) -> FileEventMap {
    let mut map = HashMap::new();
    let mut prevepoch = 0;
    for fe in swd {
        assert!(prevepoch < fe.timestamp);
        let minute = down_to_minute(fe.timestamp);
        let bin = map.entry(minute).or_insert_with(TimelineBin::new);
        (*bin).append(fe.filename);

        prevepoch = fe.timestamp;
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn parse_file_entry_checks_invalid() {
        assert!(parse_file_entry("").is_err());
        assert!(parse_file_entry("src/file.ts:2797").is_err());
        assert!(parse_file_entry("src/file.ts:2797,m").is_err());
        assert!(parse_file_entry("src/file.ts2797,1585861200:354,m").is_err());
        assert!(parse_file_entry("src/file.ts:123abc,1585861200:354,m").is_err());
        assert!(parse_file_entry("src/file.ts:123,1585861200:354,a").is_err());
    }

    #[test]
    fn parses_file_entry() {
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
    fn checks_commit_note_invalid() {
        assert!(parse_commit_note("").is_err());
        assert!(parse_commit_note("[]").is_err());
        assert!(parse_commit_note("[ver:1total:213]").is_err());
        assert!(parse_commit_note("[ver:a,total:213]").is_err());
        assert!(parse_commit_note("[ver:1,total:a]").is_err());
    }

    #[test]
    fn parses_commit_note() {
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
