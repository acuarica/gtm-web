#![feature(int_error_matching)]

use chrono::{FixedOffset, NaiveDate};
use git2::{Error, Repository};
use regex::Regex;
use serde::Serialize;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, Cursor},
    num::IntErrorKind,
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

/// Formats a git2 date time in RFC 822 format.
/// It uses the git2 time offset to proper format the date.
///
/// ```
/// use git2::Time;
/// use gtmserv::format_time;
/// assert_eq!(
///     format_time(Time::new(1589945042, 0)),
///     "2020-05-20 03:24:02 +00:00");
/// assert_eq!(
///     format_time(Time::new(1589945042, 2*60)),
///     "2020-05-20 05:24:02 +02:00");
/// assert_eq!(
///     format_time(Time::new(1589945042, -3*60)),
///     "2020-05-20 00:24:02 -03:00");
/// ```
pub fn format_time(time: git2::Time) -> String {
    use chrono::TimeZone;
    FixedOffset::east(time.offset_minutes() * 60)
        .timestamp(time.seconds(), 0)
        .to_string()
}

impl Commit {
    pub fn new(commit: &git2::Commit, project: String, note: CommitNote) -> Commit {
        let text = |msg: Option<&str>| msg.unwrap_or("<invalid utf-8>").to_string();
        Commit {
            author: text(commit.author().name()),
            date: format_time(commit.time()),
            when: format_time(commit.author().when()),
            hash: commit.id().to_string(),
            subject: text(commit.summary()),
            message: text(commit.message()),
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
/// #[macro_use] extern crate maplit;
/// assert_eq!(
///     gtmserv::parse_file_note("src/file.ts:150,1585861200:60,1585875600:90,m").unwrap(),
///     gtmserv::FileNote {
///         source_file: "src/file.ts".to_owned(),
///         time_spent: 150,
///         timeline: hashmap! {
///             "1585861200".to_owned() => 60,
///             "1585875600".to_owned() => 90,
///         },
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
///   timeline: hashmap! {
///     "1585861200".to_owned() => 354,
///     "1585875600".to_owned() => 50,
///     "1585879200".to_owned() => 240,
///     "1585908000".to_owned() => 444,
///     "1585918800".to_owned() => 1629,
///     "1585929600".to_owned() => 80,
///   },
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
        Some(first) => match VERSION_RE
            .captures_iter(
                &first
                    .expect("Could not read from cursor, aborting")
                    .to_owned(),
            )
            .next()
        {
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

/// Parses a date in `%Y-%m-%d` format.
///
/// ```
/// assert_eq!(gtmserv::parse_date("2020-05-20"), Ok(1589932800));
/// ```
pub fn parse_date(date: &str) -> chrono::ParseResult<i64> {
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d")?;
    Ok(date.and_hms(0, 0, 0).timestamp())
}

pub fn get_notes(
    result: &mut Vec<Commit>,
    repo: &Repository,
    project: String,
    from_date: i64,
    to_date: i64,
    search_message: &Option<String>,
) -> Result<(), Error> {
    let notes = repo.notes(Some(GTM_REFS))?;

    for note in notes {
        let p = note.unwrap();
        let note = repo.find_note(Some(GTM_REFS), p.1)?;
        if let Some(message) = note.message() {
            let commit = repo.find_commit(p.1)?;
            let time = commit.time().seconds() + commit.time().offset_minutes() as i64 * 60;

            let f = |msg: &String| {
                if let Some(message) = commit.message() {
                    message.to_lowercase().contains(msg.to_lowercase().as_str())
                } else {
                    true
                }
            };

            if time >= from_date
                && time <= to_date
                && match search_message {
                    None => true,
                    Some(msg) => f(msg),
                }
            {
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
///
/// If a Unix epoch is already down to the minute, `down_to_minute` returns the same value.
///
/// ```
/// assert_eq!(gtmserv::down_to_minute(1589920680), 1589920680);
/// ```
pub fn down_to_minute(timestamp: u64) -> u64 {
    (timestamp / 60) * 60
}

/// Given a Unix epoch, returns a Unix epoch rounded down to the hour.
/// It is used to create bins at the hour granularity.
///
/// # Examples
///
/// ```
/// assert_eq!(gtmserv::down_to_hour(1589673494), 1589670000);
/// ```
///
/// If a Unix epoch is already down to the hour, `down_to_hour` returns the same value.
///
/// ```
/// assert_eq!(gtmserv::down_to_hour(1589918400), 1589918400);
/// ```
pub fn down_to_hour(timestamp: u64) -> u64 {
    (timestamp / 3600) * 3600
}

pub struct Timeline {
    timeline: HashMap<u64, TimelineBin>,
}

/// ```
/// assert_eq!("", "");
/// ```
pub struct TimelineBin {
    filemap: HashMap<Filepath, usize>,
    count: usize,
}

impl TimelineBin {
    /// Creates a new `TimelineBin`.
    /// When created, the bin will be empty, *i.e.*, there are no files in it.
    pub fn new() -> TimelineBin {
        TimelineBin {
            filemap: HashMap::new(),
            count: 0,
        }
    }

    /// ```
    /// let mut bin = gtmserv::TimelineBin::new();
    /// bin.append("src/main.rs".to_owned());
    /// ```
    pub fn append(self: &mut Self, filepath: Filepath) {
        self.count += 1;
        let count = self.filemap.entry(filepath).or_insert(0);
        *count += 1;
    }

    /// ```
    /// let mut bin = gtmserv::TimelineBin::new();
    /// bin.append("src/main.rs".to_owned());
    /// assert_eq!(bin.timespent("src/main.rs".to_owned()), 60);
    /// ```
    ///
    /// When the file is not present in the bin, panics.
    ///
    /// ```should_panic
    /// let mut bin = gtmserv::TimelineBin::new();
    /// bin.timespent("src/not-present.rs".to_owned());
    /// ```
    pub fn timespent(self: &Self, filepath: Filepath) -> Seconds {
        let count = self
            .filemap
            .get(&filepath)
            .expect("File not present in bin");
        (60 * count / self.count) as Seconds
    }
}

impl Timeline {
    fn new() -> Timeline {
        Timeline {
            timeline: HashMap::new(),
        }
    }

    /// Creates a `Timeline` from a list of file events.
    ///
    /// ```
    /// use gtmserv::*;
    /// Timeline::from_events(vec![
    ///     FileEvent::new(1589673491, "src/file1.ts"),
    ///     FileEvent::new(1589673601, "test/test1.ts"),
    /// ]);
    /// ```
    ///
    /// The events in the list must be ordered by timestamp.
    ///
    /// ```should_panic
    /// use gtmserv::*;
    /// Timeline::from_events(vec![
    ///     FileEvent::new(1589673491, "src/file1.ts"),
    ///     FileEvent::new(1589673601, "test/test1.ts"),
    ///     FileEvent::new(1589673600, "test/test2.ts"),
    /// ]);
    /// ```
    pub fn from_events(events: Vec<FileEvent>) -> Timeline {
        let mut timeline = Timeline::new();
        let mut prevepoch = 0;
        for event in events {
            assert!(prevepoch < event.timestamp);
            prevepoch = event.timestamp;
            timeline.append(event);
        }

        timeline
    }

    /// Adds a new event to this timeline.
    fn append(self: &mut Self, event: FileEvent) {
        let minute = down_to_minute(event.timestamp);
        let bin = self.timeline.entry(minute).or_insert_with(TimelineBin::new);
        (*bin).append(event.filename.to_owned());
    }

    /// ```
    /// use gtmserv::*;
    /// let map = Timeline::from_events(vec![
    ///     FileEvent::new(1589673491, "src/file1.ts"),
    ///     FileEvent::new(1589673494, "src/file2.ts"),
    ///     FileEvent::new(1589673601, "test/test1.ts"),
    ///     FileEvent::new(1589673632, "test/test2.ts"),
    ///     FileEvent::new(1589673658, "assets/logo.png"),
    ///     FileEvent::new(1589673732, "assets/main.css"),
    ///     FileEvent::new(1589673854, "src/file2.ts"),
    /// ]);
    ///
    /// let bin = map.get(&1589673480).unwrap();
    /// assert_eq!(bin.timespent("src/file1.ts".to_string()), 30);
    /// assert_eq!(bin.timespent("src/file2.ts".to_string()), 30);
    ///
    /// let bin = map.get(&1589673600).unwrap();
    /// assert_eq!(bin.timespent("test/test1.ts".to_string()), 20);
    /// assert_eq!(bin.timespent("test/test2.ts".to_string()), 20);
    /// assert_eq!(bin.timespent("assets/logo.png".to_string()), 20);
    ///
    /// let bin = map.get(&1589673720).unwrap();
    /// assert_eq!(bin.timespent("assets/main.css".to_string()), 60);
    ///
    /// let bin = map.get(&1589673840).unwrap();
    /// assert_eq!(bin.timespent("src/file2.ts".to_string()), 60);
    /// ```
    pub fn get(self: &Self, timestamp: &u64) -> Option<&TimelineBin> {
        self.timeline.get(timestamp)
    }

    ///
    /// ```
    /// use gtmserv::*;
    ///
    /// let map = Timeline::from_events(vec![
    ///     FileEvent::new(1589673491, "src/file1.ts"),
    ///     FileEvent::new(1589673494, "src/file2.ts"),
    ///     FileEvent::new(1589673601, "test/test1.ts"),
    ///     FileEvent::new(1589673632, "test/test2.ts"),
    ///     FileEvent::new(1589673658, "assets/logo.png"),
    ///     FileEvent::new(1589673732, "assets/main.css"),
    /// ]);
    ///
    /// let bin = map.get(&1589673480).unwrap();
    /// assert_eq!(bin.timespent("src/file1.ts".to_string()), 30);
    /// assert_eq!(bin.timespent("src/file2.ts".to_string()), 30);
    ///
    /// let bin = map.get(&1589673600).unwrap();
    /// assert_eq!(bin.timespent("test/test1.ts".to_string()), 20);
    /// assert_eq!(bin.timespent("test/test2.ts".to_string()), 20);
    /// assert_eq!(bin.timespent("assets/logo.png".to_string()), 20);
    ///
    /// let bin = map.get(&1589673720).unwrap();
    /// assert_eq!(bin.timespent("assets/main.css".to_string()), 60);
    ///
    /// let commit_note = map.commit_note();
    /// assert_eq!(commit_note.total, 180);
    /// assert!(commit_note.files.contains(
    ///     &parse_file_note("test/test1.ts:20,1589673600:20,r").unwrap()
    ///     ));
    /// ```
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
                time_spent: tl.0,
                timeline: tl.1,
            };
            cn.files.push(note);
        }

        cn
    }
}
