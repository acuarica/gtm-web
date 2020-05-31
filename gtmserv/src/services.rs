use crate::{get_notes, projects::InitProjects, NotesFilter};
use git2::Repository;
use serde::ser::{SerializeSeq, Serializer};
use std::{
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
    let mut ser = serde_json::Serializer::new(writer);
    let mut seq = ser.serialize_seq(None).unwrap();

    for path in project_paths {
        let pkey = path
            .as_ref()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        let repo = Repository::open(path)?;
        get_notes(
            |c| {
                seq.serialize_element(&c.commit)
                    .expect("Could not serialize commit");
            },
            &repo,
            pkey,
            filter,
        )
        .unwrap();
    }
    seq.end().expect("Could not end serialize commits");

    Ok(())
}

pub fn write_project_list<W: Write>(mut writer: W, projects: &InitProjects) {
    let projects: Vec<&String> = projects.get_project_list().collect();
    let json = serde_json::to_string(&projects).unwrap();
    write!(writer, "{}", json).unwrap();
}

pub fn config_path() -> Option<PathBuf> {
    let mut path = dirs::home_dir()?;
    path.push(".git-time-metric");
    path.push("project.json");
    Some(path)
}
