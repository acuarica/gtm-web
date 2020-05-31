
use crate::{seconds, epoch, FileNote, CommitNote};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
pub struct FileEvent {
    pub timestamp: epoch,
    filename: String,
}

impl FileEvent {
    pub fn new(timestamp: epoch, filename: &str) -> FileEvent {
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
/// use gtmserv::status::*;
/// assert_eq!(down_to_minute(1589673494), 1589673480);
/// ```
///
/// If a Unix epoch is already down to the minute, `down_to_minute` returns the same value.
///
/// ```
/// use gtmserv::status::*;
/// assert_eq!(down_to_minute(1589920680), 1589920680);
/// ```
pub fn down_to_minute(timestamp: epoch) -> epoch {
    (timestamp / 60) * 60
}

/// Given a Unix epoch, returns a Unix epoch rounded down to the hour.
/// It is used to create bins at the hour granularity.
///
/// # Examples
///
/// ```
/// use gtmserv::status::*;
/// assert_eq!(down_to_hour(1589673494), 1589670000);
/// ```
///
/// If a Unix epoch is already down to the hour, `down_to_hour` returns the same value.
///
/// ```
/// use gtmserv::status::*;
/// assert_eq!(down_to_hour(1589918400), 1589918400);
/// ```
pub fn down_to_hour(timestamp: epoch) -> epoch {
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
    /// use gtmserv::status::*;
    /// let mut bin = TimelineBin::new();
    /// bin.append("src/main.rs");
    /// ```
    pub fn append(self: &mut Self, filepath: &'a str) {
        self.count += 1;
        let count = self.filemap.entry(filepath).or_insert(0);
        *count += 1;
    }

    /// ```
    /// use gtmserv::status::*;
    /// let mut bin = TimelineBin::new();
    /// bin.append("src/main.rs");
    /// assert_eq!(bin.timespent("src/main.rs"), 60);
    /// ```
    ///
    /// When the file is not present in the bin, panics.
    ///
    /// ```should_panic
    /// use gtmserv::status::*;
    /// let mut bin = TimelineBin::new();
    /// bin.timespent("src/not-present.rs");
    /// ```
    pub fn timespent(self: &Self, filepath: &str) -> seconds {
        let count = self
            .filemap
            .get(&filepath)
            .expect("File not present in bin");
        (60 * count / self.count) as seconds
    }
}

pub struct Timeline<'a> {
    timeline: HashMap<epoch, TimelineBin<'a>>,
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
    /// use gtmserv::status::*;
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
    /// use gtmserv::status::*;
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
    /// use gtmserv::status::*;
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
    pub fn get(self: &Self, timestamp: &epoch) -> Option<&TimelineBin> {
        self.timeline.get(timestamp)
    }

    ///
    /// ```
    /// #[macro_use] extern crate maplit;
    /// use gtmserv::{*, status::*};
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
    ///     &FileNote{ source_file: "test/test1.ts", time_spent: 20, timeline: btreemap! { 1589673600=>20}, status: "r" }
    ///     ));
    /// ```
    pub fn commit_note(self) -> CommitNote<'a> {
        let mut cn = CommitNote::new(1, 0);
        let mut fs = HashMap::new();
        for (ts, bin) in &self.timeline {
            for (f, _count) in &bin.filemap {
                let (timespent, e) = fs.entry(f).or_insert((0, BTreeMap::new()));
                let h = down_to_hour(*ts);
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
                status: "r",
                time_spent: tl.0,
                timeline: tl.1,
            };
            cn.files.push(note);
        }

        cn
    }
}