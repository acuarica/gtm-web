use crate::{epoch, seconds, CommitNote, FileNote};
use regex::Regex;
use std::{collections::BTreeMap, num::IntErrorKind};

/// Parses a key-value in "key:value" format.
///
/// ```
/// use gtm::parse::*;
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
    /// use gtm::parse::*;
    /// assert_eq!(parse_file_note("src/file.ts:2797"), Err(FileNoteParseError::NotEnoughEntries));
    /// assert_eq!(parse_file_note("src/file.ts:2797,m"), Err(FileNoteParseError::NotEnoughEntries));
    /// ```
    NotEnoughEntries,

    /// Occurs when it is not possible to parse the file path of this note.
    ///
    /// ```
    /// use gtm::parse::*;
    /// assert_eq!(parse_file_note("src/file.ts2797,1585861200:354,m"), Err(FileNoteParseError::UnrecognizedFilepath));
    /// assert_eq!(parse_file_note(""), Err(FileNoteParseError::UnrecognizedFilepath));
    /// ```
    UnrecognizedFilepath,

    /// Occurs when it is not possible to parse a timeline entry of this file note.
    ///
    /// ```
    /// use gtm::parse::*;
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
    /// use gtm::parse::*;
    /// assert_eq!(parse_file_note("src/file.ts:123abc,1585861200:354,m"), Err(FileNoteParseError::InvalidTotalTimespent { kind: IntErrorKind::InvalidDigit }));
    /// ```
    InvalidTotalTimespent { kind: IntErrorKind },

    /// Occurs when the status of the parsed file note is not recognized.
    ///
    /// ```
    /// use gtm::parse::*;
    /// assert_eq!(parse_file_note("src/file.ts:123,1585861200:354,a"), Err(FileNoteParseError::StatusNotRecognized { got: "a".to_owned() }));
    /// ```
    StatusNotRecognized { got: String },
}

/// Parses a file note entry.
///
/// ```
/// #[macro_use] extern crate maplit;
/// use gtm::{*, parse::*};
///
/// assert_eq!(
///     parse_file_note("src/file.ts:150,1585861200:60,1585875600:90,m").unwrap(),
///     FileNote {
///         source_file: "src/file.ts",
///         time_spent: 150,
///         timeline: btreemap! {
///             1585861200 => 60,
///             1585875600 => 90,
///         },
///         status: "m",
///     }
/// );
///
/// assert_eq!(
///  parse_file_note("comment/src/comment.ts:2797,1585861200:354,1585875600:50,1585879200:240,1585908000:444,1585918800:1629,1585929600:80,m").unwrap(),
///  FileNote {
///   source_file: "comment/src/comment.ts",
///   time_spent: 2797,
///   timeline: btreemap! {
///     1585861200 => 354,
///     1585875600 => 50,
///     1585879200 => 240,
///     1585908000 => 444,
///     1585918800 => 1629,
///     1585929600 => 80,
///   },
///   status: "m",
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
        s @ "m" | s @ "r" | s @ "d" => s,
        got => {
            return Err(FileNoteParseError::StatusNotRecognized {
                got: got.to_owned(),
            })
        }
    };

    let mut timeline = BTreeMap::new();
    for time_entry in parts {
        let (epoch, seconds) =
            parse_key_value(time_entry).ok_or_else(|| FileNoteParseError::InvalidTimelineFormat)?;
        timeline.insert(
            epoch
                .parse::<epoch>()
                .map_err(|_| FileNoteParseError::InvalidTimelineFormat)?,
            seconds
                .parse::<seconds>()
                .map_err(|err| FileNoteParseError::InvalidTimespent {
                    kind: err.kind().to_owned(),
                })?,
        );
    }
    if timeline.len() == 0 {
        return Err(FileNoteParseError::NotEnoughEntries);
    }

    let note = FileNote {
        source_file: file_name,
        time_spent: time_spent.parse::<seconds>().map_err(|err| {
            FileNoteParseError::InvalidTotalTimespent {
                kind: err.kind().to_owned(),
            }
        })?,
        timeline,
        status,
    };
    Ok(note)
}

#[derive(Debug, PartialEq)]
/// Represents the errors reported by `parse_commit_note`.
pub enum CommitNoteParseError {
    /// ```
    /// use gtm::parse::*;
    /// assert_eq!(parse_commit_note(""), Err(CommitNoteParseError::EmptyNote));
    /// ```
    EmptyNote,

    /// ```
    /// use gtm::parse::*;
    /// assert_eq!(parse_commit_note("[]"), Err(CommitNoteParseError::InvalidHeader));
    /// assert_eq!(parse_commit_note("[ver:1total:213]"), Err(CommitNoteParseError::InvalidHeader));
    /// assert_eq!(parse_commit_note("[ver:1,total:a]"), Err(CommitNoteParseError::InvalidHeader));
    /// ```
    InvalidHeader,

    /// ```
    /// use gtm::parse::*;
    /// assert_eq!(parse_commit_note("[ver:9123456789,total:213]"), Err(CommitNoteParseError::InvalidVersion));
    /// ```
    InvalidVersion,

    /// ```
    /// use gtm::parse::*;
    /// assert_eq!(parse_commit_note("[ver:1,total:9123456789]"), Err(CommitNoteParseError::InvalidTotal));
    /// ```
    InvalidTotal,

    InvalidFile {
        err: FileNoteParseError,
    },
}

/// Parses a `CommitNote`.
///
/// # Examples
///
/// Parses an empty `CommitNote`, *i.e.*, with no files, only header.
///
/// ```
/// use gtm::{*, parse::*};
/// assert_eq!(
///     parse_commit_note("[ver:2,total:123]").unwrap(),
///     CommitNote {
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
/// use gtm::{*, parse::*};
/// assert_eq!(
///         parse_commit_note(
///             "[ver:2,total:213]
/// closebrackets/src/closebrackets.ts:950,1585918800:510,1585922400:400,1585929600:40,r
/// text/src/char.ts:90,1585918800:90,r"
///         ).unwrap(),
///         CommitNote {
///             version: 2,
///             total: 213,
///             files: vec![
///                 FileNote {
///                     source_file: "closebrackets/src/closebrackets.ts",
///                     time_spent: 950,
///                     timeline: btreemap! {
///                         1585918800 => 510,
///                         1585922400 => 400,
///                         1585929600 => 40,
///                     },
///                     status: "r",
///                 },
///                 FileNote {
///                     source_file: "text/src/char.ts",
///                     time_spent: 90,
///                     timeline: btreemap! { 1585918800 => 90, },
///                     status: "r",
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
/// use gtm::{*, parse::*};
/// let note = parse_commit_note("[ver:1,total:4037]
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
///         FileNote {
///             source_file: "demo/demo.ts",
///             time_spent: 60,
///             timeline: btreemap! { 1585918800 => 60 },
///             status: "r",
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
                total: match parts[2].parse::<seconds>() {
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

#[cfg(test)]
mod tests {
    use super::parse_commit_note;
    use chrono::{DateTime, TimeZone, Utc};

    #[test]
    fn test() {
        let message = r#"[ver:1,total:4080]
.gtm/terminal.app:3169,1586977200:180,1587042000:360,1588255200:150,1589302800:180,1589587200:710,1589590800:120,1589594400:540,1589598000:425,1589601600:480,1590771600:24,r
hola.txt:797,1589587200:310,1589598000:415,1589601600:60,1590771600:12,d
../.git/modules/home/COMMIT_EDITMSG:60,1590771600:60,r
.zprofile:42,1588255200:30,1590771600:12,m
lala:12,1590771600:12,d"#;

        let note = parse_commit_note(message).unwrap();
        let ts: Vec<Vec<DateTime<Utc>>> = note
            .files
            .iter()
            .map(|f| f.timeline.iter().map(|t| Utc.timestamp(*t.0, 0)).collect())
            .collect();
        println!("{:?}", note);
        for entry in ts {
            println!("{:?}", entry);
        }
    }
}
