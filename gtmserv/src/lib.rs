#![feature(int_error_matching)]

use chrono::{DateTime, FixedOffset, TimeZone};
use git2::{Note, Repository};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::{hash_map, HashMap},
    fs::File,
    io,
    num::IntErrorKind,
    path::Path,
};

#[macro_use]
extern crate lazy_static;

extern crate chrono;

pub const GTM_REFS: &str = "refs/notes/gtm-data";

type Filepath = String;

/// Represents initialized projects by `gtm`.
/// It is represented by a `HashMap` where the keys are paths and
/// the values are formatted dates.
/// The keys are the repository path of the working directory of the
/// git repository.
/// The values indicates the date when the git repo was `init` by gtm.
pub struct InitProjects(HashMap<Filepath, String>);

impl InitProjects {
    pub fn from_file<P: AsRef<Path>>(filename: P) -> Result<Self, io::Error> {
        let file = File::open(filename)?;
        let map = serde_json::from_reader(file)?;
        Ok(InitProjects(map))
    }

    /// Return how many projects are being initialized.
    pub fn len(self: &Self) -> usize {
        self.0.len()
    }

    pub fn contains_project(self: &Self, project_path: &str) -> bool {
        self.0.contains_key(project_path)
    }

    ///
    pub fn get_project_list(self: &Self) -> hash_map::Keys<'_, String, String> {
        self.0.keys()
    }
}

type Seconds = u32;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
///
/// ```
/// #[macro_use] extern crate maplit;
/// use gtmserv::FileNote;
/// let file_note = FileNote {
///         source_file: "src/main.ts",
///         time_spent: 150,
///         timeline: hashmap! {
///             "1585861200".to_owned() => 60,
///             "1585875600".to_owned() => 90,
///         },
///         status: "r".to_owned(),
///     };
/// let from_json = serde_json::from_str(r#"{"SourceFile":"src/main.ts","TimeSpent":150,"Timeline":{"1585861200":60,"1585875600":90},"Status":"r"}"#);
/// assert_eq!(file_note, from_json.unwrap());
/// ```
pub struct FileNote<'a> {
    pub source_file: &'a str,
    pub time_spent: Seconds,
    pub timeline: HashMap<String, Seconds>,
    pub status: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommitNote<'a> {
    pub version: u32,
    pub total: Seconds,
    #[serde(borrow)]
    pub files: Vec<FileNote<'a>>,
}

impl CommitNote<'_> {
    fn new<'a>(version: u32, total: Seconds) -> CommitNote<'a> {
        CommitNote {
            version,
            total,
            files: Vec::new(),
        }
    }
}

#[derive(PartialEq, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct WorkdirStatus<'a> {
    pub total: Seconds,
    pub label: String,
    pub commit_note: CommitNote<'a>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Commit<'a> {
    pub author: String,
    pub date: String,
    pub when: String,
    pub hash: String,
    pub subject: String,
    pub message: String,
    pub project: String,
    #[serde(borrow)]
    pub note: CommitNote<'a>,
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
    FixedOffset::east(time.offset_minutes() * 60)
        .timestamp(time.seconds(), 0)
        .to_string()
}

impl Commit<'_> {
    pub fn new<'a>(commit: &git2::Commit, project: String, note: CommitNote<'a>) -> Commit<'a> {
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

/// Parses a key-value in "key:value" format.
///
/// ```
/// use gtmserv::parse_key_value;
/// assert_eq!(parse_key_value("src/file.ts:somevalue").unwrap(), ("src/file.ts", "somevalue"));
/// assert_eq!(parse_key_value("src/file.ts:some:value").unwrap(), ("src/file.ts", "some:value"));
/// assert_eq!(parse_key_value(":").unwrap(), ("", ""));
/// assert_eq!(parse_key_value(""), None);
/// assert_eq!(parse_key_value("keynovalue"), None);
/// ```
pub fn parse_key_value(text: &str) -> Option<(&str, &str)> {
    let mut key_value_parts = text.splitn(2, ':');
    let key = key_value_parts.next()?;
    let value = key_value_parts.next()?;
    Some((key, value))
}

#[derive(PartialEq, Debug)]
/// These are the kinds of parsing errors detected
/// when parsing a `FileNote`.
pub enum FileNoteParseError {
    /// Occurs when there are less than 3 components to parse.
    ///
    /// ```
    /// use gtmserv::*;
    /// assert_eq!(parse_file_note("src/file.ts:2797"), Err(FileNoteParseError::NotEnoughEntries));
    /// assert_eq!(parse_file_note("src/file.ts:2797,m"), Err(FileNoteParseError::NotEnoughEntries));
    /// ```
    NotEnoughEntries,

    /// Occurs when it is not possible to parse the file path of this note.
    ///
    /// ```
    /// use gtmserv::*;
    /// assert_eq!(parse_file_note("src/file.ts2797,1585861200:354,m"), Err(FileNoteParseError::UnrecognizedFilepath));
    /// assert_eq!(parse_file_note(""), Err(FileNoteParseError::UnrecognizedFilepath));
    /// ```
    UnrecognizedFilepath,

    /// Occurs when it is not possible to parse a timeline entry of this file note.
    ///
    /// ```
    /// use gtmserv::*;
    /// assert_eq!(parse_file_note("src/file.ts:2797,1585861200;354,m"), Err(FileNoteParseError::InvalidTimelineFormat));
    /// ```
    InvalidTimelineFormat,

    ///
    InvalidTimespent { kind: IntErrorKind },

    /// Occurs when the total time spent is not a valid number.
    ///
    /// ```
    /// #![feature(int_error_matching)]
    /// use std::num::IntErrorKind;
    /// use gtmserv::*;
    /// assert_eq!(parse_file_note("src/file.ts:123abc,1585861200:354,m"), Err(FileNoteParseError::InvalidTotalTimespent { kind: IntErrorKind::InvalidDigit }));
    /// ```
    InvalidTotalTimespent { kind: IntErrorKind },

    /// Occurs when the status of the parsed file note is not recognized.
    ///
    /// ```
    /// use gtmserv::*;
    /// assert_eq!(parse_file_note("src/file.ts:123,1585861200:354,a"), Err(FileNoteParseError::StatusNotRecognized { got: "a".to_owned() }));
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
///         source_file: "src/file.ts",
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
///   source_file: "comment/src/comment.ts",
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
pub fn parse_file_note<'a>(file_entry: &'a str) -> Result<FileNote<'a>, FileNoteParseError> {
    let mut parts = file_entry.split(',');
    let (file_name, time_spent) = parse_key_value(
        parts
            .next()
            .ok_or_else(|| FileNoteParseError::NotEnoughEntries)?,
    )
    .ok_or_else(|| FileNoteParseError::UnrecognizedFilepath)?;

    let status = match parts
        .next_back()
        .ok_or_else(|| FileNoteParseError::NotEnoughEntries)?
    {
        s @ "m" | s @ "r" | s @ "d" => s.to_owned(),
        got => {
            return Err(FileNoteParseError::StatusNotRecognized {
                got: got.to_owned(),
            })
        }
    };

    let mut timeline = HashMap::new();
    for time_entry in parts {
        let (epoch, seconds) =
            parse_key_value(time_entry).ok_or_else(|| FileNoteParseError::InvalidTimelineFormat)?;
        timeline.insert(
            epoch.to_owned(),
            match seconds.parse::<Seconds>() {
                Err(err) => {
                    return Err(FileNoteParseError::InvalidTimespent {
                        kind: err.kind().to_owned(),
                    })
                }
                Ok(value) => value,
            },
        );
    }
    if timeline.len() == 0 {
        return Err(FileNoteParseError::NotEnoughEntries);
    }

    let note = FileNote {
        source_file: file_name,
        time_spent: match time_spent.parse::<Seconds>() {
            Err(err) => {
                return Err(FileNoteParseError::InvalidTotalTimespent {
                    kind: err.kind().to_owned(),
                })
            }
            Ok(value) => value,
        },
        timeline,
        status,
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
///                     source_file: "closebrackets/src/closebrackets.ts",
///                     time_spent: 950,
///                     timeline: hashmap! {
///                         "1585918800".to_string() => 510,
///                         "1585922400".to_string() => 400,
///                         "1585929600".to_string() => 40,
///                     },
///                     status: "r".to_string(),
///                 },
///                 gtmserv::FileNote {
///                     source_file: "text/src/char.ts",
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
///             source_file: "demo/demo.ts",
///             time_spent: 60,
///             timeline: hashmap! { "1585918800".to_string() => 60 },
///             status: "r".to_string(),
///         }
///     );
/// ```
pub fn parse_commit_note<'a>(message: &'a str) -> Result<CommitNote<'a>, CommitNoteParseError> {
    lazy_static! {
        static ref VERSION_RE: Regex = Regex::new(r"\[ver:(\d+),total:(\d+)\]").unwrap();
    }
    let mut lines = message.lines();
    let mut commit_note = match lines.next() {
        None => return Err(CommitNoteParseError::EmptyNote),
        Some(first) => match VERSION_RE.captures_iter(first).next() {
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
        match parse_file_note(line) {
            Err(err) => return Err(CommitNoteParseError::InvalidFile { err }),
            Ok(entry) => commit_note.files.push(entry),
        };
    }
    Ok(commit_note)
}

pub fn get_commits(_path: &str) -> Result<(), git2::Error> {
    // let repo = Repository::open(path)?;
    // let nt = repo.notes(Some("refs/notes/gtm-data")).unwrap();
    Ok(())
}

type UnixEpoch = i64;

pub struct NotesFilter {
    from_date: Option<UnixEpoch>,
    to_date: Option<UnixEpoch>,
    needle: Option<String>,
}

impl NotesFilter {
    pub fn new() -> Self {
        Self {
            from_date: None,
            to_date: None,
            needle: None,
        }
    }

    pub fn from_date<T: TimeZone>(self: &mut Self, date: DateTime<T>) -> &mut Self {
        self.from_date = Some(date.timestamp());
        self
    }

    pub fn to_date<T: TimeZone>(self: &mut Self, date: DateTime<T>) -> &mut Self {
        self.to_date = Some(date.timestamp());
        self
    }

    pub fn needle<T: TimeZone>(self: &mut Self, needle: String) -> &mut Self {
        self.needle = Some(needle);
        self
    }
}

#[derive(Debug)]
pub struct GitCommitNote<'a> {
    pub commit: Commit<'a>,
    note: git2::Note<'a>,
}

pub fn get_notes<'r, F>(
    mut with: F,
    repo: &'r Repository,
    project: String,
    from_date: i64,
    to_date: i64,
    search_message: &Option<String>,
) -> Result<(), git2::Error>
where
    F: FnMut(GitCommitNote<'r>) -> (),
{
    let notes = repo.notes(Some(GTM_REFS))?;

    for note_assoc in notes {
        let p = note_assoc.unwrap();
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
            // let message: &'r str = note.message().as_ref().unwrap();
            // parse_commit_note(np.as_ref().unwrap().message().as_ref().unwrap()).unwrap()
            let note = repo.find_note(Some(GTM_REFS), p.1)?;
            // let note_message = unsafe { (&note as *const Note).as_ref().unwrap().message() };
            let note_message = unsafe { (*(&note as *const Note)).message() };
            if let Some(note_message) = note_message {
                if let Ok(commit_note) = parse_commit_note(note_message) {
                    with(GitCommitNote {
                        commit: Commit::new(&commit, project.to_owned(), commit_note),
                        note,
                    });
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
            filename: filename.to_owned(),
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

/// ```
/// assert_eq!("", "");
/// ```
pub struct TimelineBin<'a> {
    filemap: HashMap<&'a str, usize>,
    count: usize,
}

impl<'a> TimelineBin<'a> {
    /// Creates a new `TimelineBin`.
    /// When created, the bin will be empty, *i.e.*, there are no files in it.
    pub fn new() -> TimelineBin<'a> {
        TimelineBin {
            filemap: HashMap::new(),
            count: 0,
        }
    }

    /// ```
    /// let mut bin = gtmserv::TimelineBin::new();
    /// bin.append("src/main.rs");
    /// ```
    pub fn append(self: &mut Self, filepath: &'a str) {
        self.count += 1;
        let count = self.filemap.entry(filepath).or_insert(0);
        *count += 1;
    }

    /// ```
    /// let mut bin = gtmserv::TimelineBin::new();
    /// bin.append("src/main.rs");
    /// assert_eq!(bin.timespent("src/main.rs"), 60);
    /// ```
    ///
    /// When the file is not present in the bin, panics.
    ///
    /// ```should_panic
    /// let mut bin = gtmserv::TimelineBin::new();
    /// bin.timespent("src/not-present.rs");
    /// ```
    pub fn timespent(self: &Self, filepath: &str) -> Seconds {
        let count = self
            .filemap
            .get(&filepath)
            .expect("File not present in bin");
        (60 * count / self.count) as Seconds
    }
}

pub struct Timeline<'a> {
    timeline: HashMap<u64, TimelineBin<'a>>,
}

impl<'a> Timeline<'a> {
    fn new() -> Timeline<'a> {
        Timeline {
            timeline: HashMap::new(),
        }
    }

    /// Creates a `Timeline` from a list of file events.
    ///
    /// ```
    /// use gtmserv::*;
    /// let events = vec![
    ///     FileEvent::new(1589673491, "src/file1.ts"),
    ///     FileEvent::new(1589673601, "test/test1.ts"),
    /// ];
    /// Timeline::from_events(&events);
    /// ```
    ///
    /// The events in the list must be ordered by timestamp.
    ///
    /// ```should_panic
    /// use gtmserv::*;
    /// let events = vec![
    ///     FileEvent::new(1589673491, "src/file1.ts"),
    ///     FileEvent::new(1589673601, "test/test1.ts"),
    ///     FileEvent::new(1589673600, "test/test2.ts"),
    /// ];
    /// Timeline::from_events(&events);
    /// ```
    pub fn from_events(events: &'a Vec<FileEvent>) -> Timeline<'a> {
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
    fn append(self: &mut Self, event: &'a FileEvent) {
        let minute = down_to_minute(event.timestamp);
        let bin = self.timeline.entry(minute).or_insert_with(TimelineBin::new);
        (*bin).append(event.filename.as_str());
    }

    /// ```
    /// use gtmserv::*;
    /// let events = vec![
    ///     FileEvent::new(1589673491, "src/file1.ts"),
    ///     FileEvent::new(1589673494, "src/file2.ts"),
    ///     FileEvent::new(1589673601, "test/test1.ts"),
    ///     FileEvent::new(1589673632, "test/test2.ts"),
    ///     FileEvent::new(1589673658, "assets/logo.png"),
    ///     FileEvent::new(1589673732, "assets/main.css"),
    ///     FileEvent::new(1589673854, "src/file2.ts"),
    /// ];
    /// let map = Timeline::from_events(&events);
    ///
    /// let bin = map.get(&1589673480).unwrap();
    /// assert_eq!(bin.timespent("src/file1.ts"), 30);
    /// assert_eq!(bin.timespent("src/file2.ts"), 30);
    ///
    /// let bin = map.get(&1589673600).unwrap();
    /// assert_eq!(bin.timespent("test/test1.ts"), 20);
    /// assert_eq!(bin.timespent("test/test2.ts"), 20);
    /// assert_eq!(bin.timespent("assets/logo.png"), 20);
    ///
    /// let bin = map.get(&1589673720).unwrap();
    /// assert_eq!(bin.timespent("assets/main.css"), 60);
    ///
    /// let bin = map.get(&1589673840).unwrap();
    /// assert_eq!(bin.timespent("src/file2.ts"), 60);
    /// ```
    pub fn get(self: &Self, timestamp: &u64) -> Option<&TimelineBin> {
        self.timeline.get(timestamp)
    }

    ///
    /// ```
    /// use gtmserv::*;
    /// let events = vec![
    ///     FileEvent::new(1589673491, "src/file1.ts"),
    ///     FileEvent::new(1589673494, "src/file2.ts"),
    ///     FileEvent::new(1589673601, "test/test1.ts"),
    ///     FileEvent::new(1589673632, "test/test2.ts"),
    ///     FileEvent::new(1589673658, "assets/logo.png"),
    ///     FileEvent::new(1589673732, "assets/main.css"),
    /// ];
    /// let map = Timeline::from_events(&events);
    ///
    /// let bin = map.get(&1589673480).unwrap();
    /// assert_eq!(bin.timespent("src/file1.ts"), 30);
    /// assert_eq!(bin.timespent("src/file2.ts"), 30);
    ///
    /// let bin = map.get(&1589673600).unwrap();
    /// assert_eq!(bin.timespent("test/test1.ts"), 20);
    /// assert_eq!(bin.timespent("test/test2.ts"), 20);
    /// assert_eq!(bin.timespent("assets/logo.png"), 20);
    ///
    /// let bin = map.get(&1589673720).unwrap();
    /// assert_eq!(bin.timespent("assets/main.css"), 60);
    ///
    /// let commit_note = map.commit_note();
    /// assert_eq!(commit_note.total, 180);
    /// assert!(commit_note.files.contains(
    ///     &parse_file_note("test/test1.ts:20,1589673600:20,r").unwrap()
    ///     ));
    /// ```
    pub fn commit_note(self) -> CommitNote<'a> {
        let mut cn = CommitNote::new(1, 0);
        let mut fs = HashMap::new();
        for (ts, bin) in &self.timeline {
            for (f, _count) in &bin.filemap {
                let (timespent, e) = fs.entry(f).or_insert((0, HashMap::new()));
                let h = down_to_hour(*ts).to_string();
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
