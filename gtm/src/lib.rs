use git2::{Error, Repository};
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::Cursor;

#[macro_use]
extern crate lazy_static;

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

pub fn read_projects(filename: &str) -> Result<InitProjects, std::io::Error> {
    let file = File::open(filename)?;
    let ps = serde_json::from_reader(file)?;
    Ok(ps)
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
        timeline: timeline,
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

pub fn commits() -> Result<(), Error> {
    println!("asdf");

    let repo = Repository::open("/Users/luigi/work/#forks/codemirror.next")?;
    let r = repo.find_remote("origin").unwrap();
    println!("remote: {:#?}", r.url());

    let nt = repo.notes(Some("refs/notes/gtm-data")).unwrap();

    for n in nt {
        let p = n.unwrap();
        //let c = repo.find_commit( p.0).unwrap();
        let note = repo.find_note(Some("refs/notes/gtm-data"), p.1);
        println!("note: {:?}", p);
        println!("{}", note.unwrap().message().unwrap());

        // let message = note.unwrap().message().unwrap();
        // notes(message)
    }

    return Ok(());
}
