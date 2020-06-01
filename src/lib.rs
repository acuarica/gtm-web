#![feature(int_error_matching)]

#[macro_use]
extern crate lazy_static;

use chrono::{DateTime, FixedOffset, TimeZone};
use git2::{Note, Repository};
use parse::parse_commit_note;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

extern crate chrono;

pub const GTM_REFS: &str = "refs/notes/gtm-data";

pub mod clone;
pub mod parse;
pub mod projects;
pub mod services;
pub mod status;

/// Represents a Unix epoch (timestamp), *i.e.*, number of non-leap
/// seconds elapsed since January 1st, 1970, 0:00:00 UTC.
#[allow(non_camel_case_types)]
pub type epoch = i64;

#[allow(non_camel_case_types)]
type seconds = u32;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Contains the access timeline of a file within a commit note.
/// The sum of all timeline entries should be equal to the `time_spent` field.
///
/// # Serialization and Deserialization
///
/// Serialization and deserialization use pascal case.
/// Because the `FileNote` struct uses `BTreeMap` to hold the file timeline,
/// the order of timeline entries is preserved when serializing and deserializing.
///
/// ```
/// #[macro_use] extern crate maplit;
/// use gtm::*;
///
/// assert_eq!(serde_json::to_string(&FileNote {
///         source_file: "src/main.ts",
///         time_spent: 150,
///         timeline: btreemap! { 1585861200 => 60, 1585875600 => 90 },
///         status: "r",
///     }).unwrap(),
///     r#"{"SourceFile":"src/main.ts","TimeSpent":150,"Timeline":{"1585861200":60,"1585875600":90},"Status":"r"}"#
/// );
/// ```
///
/// Note that when using deserialization in JSON format,
/// timeline keys will be automatically converted from `&str` keys.
///
/// ```
/// #[macro_use] extern crate maplit;
/// use gtm::*;
///
/// assert_eq!(FileNote {
///         source_file: "src/main.ts",
///         time_spent: 150,
///         timeline: btreemap! { 1585861200 => 60, 1585875600 => 90 },
///         status: "r",
///     },
///     serde_json::from_str(
///         r#"{"SourceFile":"src/main.ts","TimeSpent":150,"Timeline":{"1585861200":60,"1585875600":90},"Status":"r"}"#
///     ).unwrap());
/// ```
pub struct FileNote<'a> {
    pub source_file: &'a str,
    pub time_spent: seconds,
    pub timeline: BTreeMap<epoch, seconds>,
    pub status: &'a str,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommitNote<'a> {
    pub version: u32,
    pub total: seconds,
    #[serde(borrow)]
    pub files: Vec<FileNote<'a>>,
}

impl CommitNote<'_> {
    pub fn new<'a>(version: u32, total: seconds) -> CommitNote<'a> {
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
    pub total: seconds,
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
/// use gtm::format_time;
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
        let mut msg = commit
            .message()
            .unwrap_or("<invalid utf-8>")
            .splitn(2, "\n\n");
        Commit {
            author: text(commit.author().name()),
            date: format_time(commit.time()),
            when: format_time(commit.author().when()),
            hash: commit.id().to_string(),
            subject: msg.next().unwrap().to_owned(),
            message: msg.next().unwrap_or("").to_owned(),
            project,
            note,
        }
    }
}

pub fn get_commits(_path: &str) -> Result<(), git2::Error> {
    // let repo = Repository::open(path)?;
    // let nt = repo.notes(Some("refs/notes/gtm-data")).unwrap();
    Ok(())
}

pub struct NotesFilter {
    pub from: Option<epoch>,
    pub to: Option<epoch>,
    pub needle: Option<String>,
}

impl NotesFilter {

    pub fn all() -> Self {
        Self {
            from: None,
            to: None,
            needle: None,
        }
    }

    pub fn from<T: TimeZone>(&mut self, date: DateTime<T>) -> &mut Self {
        self.from = Some(date.timestamp());
        self
    }

    pub fn to<T: TimeZone>(&mut self, date: DateTime<T>) -> &mut Self {
        self.to = Some(date.timestamp());
        self
    }

    pub fn needle<T: TimeZone>(&mut self, needle: String) -> &mut Self {
        self.needle = Some(needle);
        self
    }

    fn filter(&self, commit: &git2::Commit) -> bool {
        let time = commit.time().seconds() + commit.time().offset_minutes() as i64 * 60;
        self.from.map_or(true, |from| time >= from)
            && self.to.map_or(true, |to| time <= to)
            && self.needle.as_ref().map_or(true, |msg: &String| {
                if let Some(message) = commit.message() {
                    message.to_lowercase().contains(msg.to_lowercase().as_str())
                } else {
                    true
                }
            })
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
    project: &str,
    filter: &NotesFilter,
) -> Result<(), git2::Error>
where
    F: FnMut(GitCommitNote<'r>) -> (),
{
    let notes = repo.notes(Some(GTM_REFS))?;

    for note_assoc in notes {
        let p = note_assoc.unwrap();
        let commit = repo.find_commit(p.1)?;

        if filter.filter(&commit) {
            let note = repo.find_note(Some(GTM_REFS), p.1)?;
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
