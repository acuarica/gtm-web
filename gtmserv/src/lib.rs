#![feature(int_error_matching)]

use chrono::NaiveDate;
use chrono::ParseResult;
use git2::{Error, Repository};
use regex::Regex;
use serde::Serialize;
use std::num::IntErrorKind;
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
/// The values indicates the date when the git repo was `init` by gtm.
type InitProjects = HashMap<Filepath, String>;

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

impl CommitNote {
    fn new(version: u32, total: Seconds) -> CommitNote {
        CommitNote {
            version,
            total,
            files: Vec::new(),
        }
    }
}

#[derive(PartialEq, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct WorkdirStatus {
    pub total: Seconds,
    pub label: String,
    pub commit_note: CommitNote,
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

#[derive(PartialEq, Debug)]
/// These are the kinds of parsing errors detected
/// when parsing a `FileNote`.
pub enum FileNoteParseError {
    /// Occurs when there are less than 3 components to parse.
    ///
    /// ```
    /// assert_eq!(gtmserv::parse_file_note(""), Err(gtmserv::FileNoteParseError::NotEnoughEntries));
    /// assert_eq!(gtmserv::parse_file_note("src/file.ts:2797"), Err(gtmserv::FileNoteParseError::NotEnoughEntries));
    /// assert_eq!(gtmserv::parse_file_note("src/file.ts:2797,m"), Err(gtmserv::FileNoteParseError::NotEnoughEntries));
    /// ```
    NotEnoughEntries,

    /// Occurs when it is not possible to parse the file path of this note.
    /// ```
    /// assert_eq!(gtmserv::parse_file_note("src/file.ts2797,1585861200:354,m"), Err(gtmserv::FileNoteParseError::UnrecognizedFilepath));
    /// ```
    UnrecognizedFilepath,

    ///
    InvalidTimelineFormat,

    ///
    InvalidTimespent { kind: IntErrorKind },

    /// Occurs when the total time spent is not a valid number.
    ///
    /// ```
    /// #![feature(int_error_matching)]
    /// use std::num::IntErrorKind;
    /// assert_eq!(gtmserv::parse_file_note("src/file.ts:123abc,1585861200:354,m"), Err(gtmserv::FileNoteParseError::InvalidTotalTimespent { kind: IntErrorKind::InvalidDigit }));
    /// ```
    InvalidTotalTimespent { kind: IntErrorKind },

    /// Occurs when the status of the parsed file note is not recognized.
    ///
    /// ```
    /// assert_eq!(gtmserv::parse_file_note("src/file.ts:123,1585861200:354,a"), Err(gtmserv::FileNoteParseError::StatusNotRecognized { got: "a".to_owned() }));
    /// ```
    StatusNotRecognized { got: String },
}

/// Parses a file note entry.
///
/// ```
/// assert_eq!(
///     gtmserv::parse_file_note("src/file.ts:150,1585861200:60,1585875600:90,m").unwrap(),
///     gtmserv::FileNote {
///         source_file: "src/file.ts".to_owned(),
///         time_spent: 150,
///         timeline: [
///             ("1585861200".to_owned(), 60),
///             ("1585875600".to_owned(), 90)
///         ]
///         .iter()
///         .cloned()
///         .collect(),
///         status: "m".to_owned(),
///     }
/// );
///
/// assert_eq!(
///  gtmserv::parse_file_note("comment/src/comment.ts:2797,1585861200:354,1585875600:50,1585879200:240,1585908000:444,1585918800:1629,1585929600:80,m")
///   .unwrap(),
///  gtmserv::FileNote {
///   source_file: "comment/src/comment.ts".to_string(),
///   time_spent: 2797,
///   timeline: [
///     ("1585861200".to_owned(), 354),
///     ("1585875600".to_owned(), 50),
///     ("1585879200".to_owned(), 240),
///     ("1585908000".to_owned(), 444),
///     ("1585918800".to_owned(), 1629),
///     ("1585929600".to_owned(), 80),
///   ]
///   .iter()
///   .cloned()
///   .collect(),
///   status: "m".to_string(),
/// });
/// ```
pub fn parse_file_note(file_entry: &str) -> Result<FileNote, FileNoteParseError> {
    let parts: Vec<&str> = file_entry.split(',').collect();
    if parts.len() < 3 {
        return Err(FileNoteParseError::NotEnoughEntries);
    }
    let file_name: Vec<&str> = parts[0].split(':').collect();
    if file_name.len() < 2 {
        return Err(FileNoteParseError::UnrecognizedFilepath);
    }

    let mut timeline = HashMap::new();
    for i in 1..parts.len() - 1 {
        let timeline_entry: Vec<&str> = parts[i].split(':').collect();
        if timeline_entry.len() != 2 {
            return Err(FileNoteParseError::InvalidTimelineFormat);
        }
        timeline.insert(
            timeline_entry[0].to_string(),
            match timeline_entry[1].parse::<Seconds>() {
                Err(err) => {
                    return Err(FileNoteParseError::InvalidTimespent {
                        kind: err.kind().to_owned(),
                    })
                }
                Ok(value) => value,
            },
        );
    }

    let note = FileNote {
        source_file: file_name[0].to_string(),
        time_spent: match file_name[1].parse::<Seconds>() {
            Err(err) => {
                return Err(FileNoteParseError::InvalidTotalTimespent {
                    kind: err.kind().to_owned(),
                })
            }
            Ok(value) => value,
        },
        timeline,
        status: match *parts.last().unwrap() {
            s @ "m" | s @ "r" | s @ "d" => s.to_owned(),
            got => {
                return Err(FileNoteParseError::StatusNotRecognized {
                    got: got.to_owned(),
                })
            }
        },
    };
    Ok(note)
}

#[derive(Debug, PartialEq)]
/// Represents the errors reported by `parse_commit_note`.
pub enum CommitNoteParseError {
    /// ```
    /// assert_eq!(gtmserv::parse_commit_note(""), Err(gtmserv::CommitNoteParseError::EmptyNote));
    /// ```
    EmptyNote,

    ///
    NonUtf8,

    /// ```
    /// assert_eq!(gtmserv::parse_commit_note("[]"), Err(gtmserv::CommitNoteParseError::InvalidHeader));
    /// assert_eq!(gtmserv::parse_commit_note("[ver:1total:213]"), Err(gtmserv::CommitNoteParseError::InvalidHeader));
    /// assert_eq!(gtmserv::parse_commit_note("[ver:1,total:a]"), Err(gtmserv::CommitNoteParseError::InvalidHeader));
    /// ```
    InvalidHeader,

    /// ```
    /// assert_eq!(gtmserv::parse_commit_note("[ver:9123456789,total:213]"), Err(gtmserv::CommitNoteParseError::InvalidVersion));
    /// ```
    InvalidVersion,

    /// ```
    /// assert_eq!(gtmserv::parse_commit_note("[ver:1,total:9123456789]"), Err(gtmserv::CommitNoteParseError::InvalidTotal));
    /// ```
    InvalidTotal,

    InvalidFile {
        err: FileNoteParseError,
    },
}

/// Parses a `CommitNote`.
/// 
/// # Example
/// 
/// ```
/// assert_eq!(
///     gtmserv::parse_commit_note("[ver:2,total:123]").unwrap(),
///     gtmserv::CommitNote {
///         version: 2,
///         total: 123,
///         files: Vec::new(),
///     }
/// );
/// ```
/// 
/// A more contrived example:
/// 
/// ```
/// #[macro_use] extern crate maplit;
/// assert_eq!(
///         gtmserv::parse_commit_note(
///             "[ver:2,total:213]
/// closebrackets/src/closebrackets.ts:950,1585918800:510,1585922400:400,1585929600:40,r
/// text/src/char.ts:90,1585918800:90,r"
///         ).unwrap(),
///         gtmserv::CommitNote {
///             version: 2,
///             total: 213,
///             files: vec![
///                 gtmserv::FileNote {
///                     source_file: "closebrackets/src/closebrackets.ts".to_string(),
///                     time_spent: 950,
///                     timeline: hashmap! {
///                         "1585918800".to_string() => 510,
///                         "1585922400".to_string() => 400,
///                         "1585929600".to_string() => 40,
///                     },
///                     status: "r".to_string(),
///                 },
///                 gtmserv::FileNote {
///                     source_file: "text/src/char.ts".to_string(),
///                     time_spent: 90,
///                     timeline: hashmap! { "1585918800".to_string() => 90, },
///                     status: "r".to_string(),
///                 }
///             ],
///         }
///     );
/// ```
/// 
/// We can also check for individual files:
/// 
/// ```
/// #[macro_use] extern crate maplit;
/// let note = gtmserv::parse_commit_note("[ver:1,total:4037]
/// comment/src/comment.ts:2797,1585861200:354,1585875600:50,1585879200:240,1585908000:444,1585918800:1629,1585929600:80,m
/// closebrackets/src/closebrackets.ts:950,1585918800:510,1585922400:400,1585929600:40,r
/// text/src/char.ts:90,1585918800:90,r
/// demo/demo.ts:60,1585918800:60,r
/// state/src/selection.ts:40,1585918800:40,r
/// highlight/src/highlight.ts:30,1585918800:30,r
/// lang-javascript/src/javascript.ts:30,1585918800:30,r
/// node_modules/w3c-keyname/index.d.ts:20,1585922400:20,r
/// CHANGELOG.md:20,1585918800:20,r").unwrap();
///     assert_eq!(note.version, 1);
///     assert_eq!(note.total, 4037);
///     assert_eq!(note.files.len(), 9);
///     assert_eq!(
///         note.files[3],
///         gtmserv::FileNote {
///             source_file: "demo/demo.ts".to_string(),
///             time_spent: 60,
///             timeline: hashmap! { "1585918800".to_string() => 60 },
///             status: "r".to_string(),
///         }
///     );
/// ```
pub fn parse_commit_note(message: &str) -> Result<CommitNote, CommitNoteParseError> {
    lazy_static! {
        static ref VERSION_RE: Regex = Regex::new(r"\[ver:(\d+),total:(\d+)\]").unwrap();
    }
    let mut lines = Cursor::new(message).lines();
    let mut commit = match lines.next() {
        None => return Err(CommitNoteParseError::EmptyNote),
        Some(Err(_)) => return Err(CommitNoteParseError::NonUtf8),
        Some(Ok(first)) => match VERSION_RE.captures_iter(&first.to_owned()).next() {
            None => return Err(CommitNoteParseError::InvalidHeader),
            Some(parts) => CommitNote {
                version: match parts[1].parse::<u32>() {
                    Err(_) => return Err(CommitNoteParseError::InvalidVersion),
                    Ok(value) => value,
                },
                total: match parts[2].parse::<Seconds>() {
                    Err(_) => return Err(CommitNoteParseError::InvalidTotal),
                    Ok(value) => value,
                },
                files: Vec::new(),
            },
        },
    };
    for line in lines {
        match parse_file_note(&line.unwrap()) {
            Err(err) => return Err(CommitNoteParseError::InvalidFile { err }),
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

#[derive(Debug)]
pub struct FileEvent {
    pub timestamp: u64,
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

pub fn down_to_hour(timestamp: u64) -> u64 {
    (timestamp / 3600) * 3600
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

pub struct Timeline {
    timeline: HashMap<u64, TimelineBin>,
}

impl Timeline {
    pub fn new() -> Timeline {
        Timeline {
            timeline: HashMap::new(),
        }
    }

    pub fn append(self: &mut Self, fileevent: FileEvent) {
        let minute = down_to_minute(fileevent.timestamp);
        let bin = self.timeline.entry(minute).or_insert_with(TimelineBin::new);
        (*bin).append(fileevent.filename.to_owned());
    }

    pub fn get(self: &Self, timestamp: &u64) -> Option<&TimelineBin> {
        self.timeline.get(timestamp)
    }

    pub fn commit_note(self) -> CommitNote {
        let mut cn = CommitNote::new(1, 0);
        let mut fs = HashMap::new();
        for (ts, bin) in self.timeline {
            for (f, _count) in &bin.filemap {
                let (timespent, e) = fs.entry(f.to_owned()).or_insert((0, HashMap::new()));
                let h = down_to_hour(ts).to_string();
                let t = (*e).entry(h).or_insert(0);
                let seconds = bin.timespent(f.to_owned());
                *timespent += seconds;
                *t += seconds;
                cn.total += seconds;
            }
        }

        for (fp, tl) in fs {
            let note = FileNote {
                source_file: fp,
                status: "r".to_string(),
                time_spent: 0,
                timeline: tl.1,
            };
            cn.files.push(note);
        }

        cn
    }
}

/// Creates a file event map.
pub fn get_status(swd: StatusWorkdir) -> Timeline {
    let mut timeline = Timeline::new();
    let mut prevepoch = 0;
    for fileevent in swd {
        assert!(prevepoch < fileevent.timestamp);
        prevepoch = fileevent.timestamp;
        timeline.append(fileevent);
    }

    timeline
}

pub fn read_status() {}
