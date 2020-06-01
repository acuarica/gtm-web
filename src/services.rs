use crate::{
    get_notes,
    projects::ProjectKey,
    projects::Projects,
    status::{FileEvent, Timeline},
    NotesFilter, WorkdirStatus,
};
use fs::read_to_string;
use git2::Repository;
use serde::ser::Serializer;
use std::{
    fs::{self, read_dir},
    io::Write,
    path::{Path, PathBuf},
};

pub fn write_commits<W: Write, I: Iterator>(
    writer: &mut W,
    project_paths: I,
    filter: &NotesFilter,
) -> Result<(), git2::Error>
where
    I::Item: AsRef<Path>,
{
    use serde::ser::SerializeSeq;

    let mut ser = serde_json::Serializer::new(writer);
    let mut seq = ser.serialize_seq(None).unwrap();

    for path in project_paths {
        let repo = Repository::open(&path)?;
        get_notes(
            |c| {
                seq.serialize_element(&c.commit)
                    .expect("Could not serialize commit");
            },
            &repo,
            path.as_ref().key(),
            filter,
        )
        .unwrap();
    }
    seq.end().expect("Could not end serialize commits");

    Ok(())
}

pub fn write_project_list<W: Write>(mut writer: W, projects: &Projects) {
    let projects: Vec<&PathBuf> = projects.keys().collect();
    let json = serde_json::to_string(&projects).unwrap();
    write!(writer, "{}", json).unwrap();
}

pub fn write_workdir_status<W: Write, I: Iterator>(writer: &mut W, project_paths: I)
where
    I::Item: AsRef<Path>,
{
    use serde::ser::SerializeMap;

    let mut ser = serde_json::Serializer::new(writer);
    let mut map = ser
        .serialize_map(None)
        .expect("Could not start serialize workdir status");
    for project in project_paths {
        let mut path = PathBuf::new();
        path.push(&project);
        path.push(".gtm");
        let entries = read_dir(path).unwrap();
        let mut events = Vec::new();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if !path.is_dir() && path.extension().unwrap() == "event" {
                let ts: &str = path.file_stem().unwrap().to_str().unwrap();
                let ts = ts.parse().unwrap();
                let filepath = read_to_string(path).unwrap();
                let fe = FileEvent::new(ts, filepath.as_ref());
                events.push(fe);
            }
        }
        events.sort_by_key(|k| k.timestamp);
        let cn = Timeline::from_events(&events).commit_note();
        let ws = WorkdirStatus {
            total: cn.total,
            label: "TBD".to_string(),
            commit_note: cn,
        };

        map.serialize_entry(&project.key(), &ws)
            .expect("Write workdir status failed");
    }
    map.end().expect("Could not end serialize workdir status");
}
