use std::{
    collections::{btree_map, BTreeMap},
    fs::File,
    io,
    path::Path,
};

type Filepath = String;

/// Represents initialized projects by `gtm`.
/// It is represented by a `BTreeMap` where the keys are paths and
/// the values are formatted dates.
/// The keys are the repository path of the working directory of the
/// git repository.
/// The values indicates the date when the git repo was `init` by gtm.
pub struct InitProjects(BTreeMap<Filepath, String>);

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
    pub fn get_project_list(self: &Self) -> btree_map::Keys<'_, String, String> {
        self.0.keys()
    }
}
