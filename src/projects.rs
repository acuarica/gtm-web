use std::{
    collections::{btree_map, BTreeMap},
    fs::File,
    io,
    path::{Path, PathBuf},
};

/// Represents local projects initialized by `gtm`.
/// These project live in the host system of the user.
///
/// It is represented by a `BTreeMap`.
/// The keys are the repository path of the working directory of the
/// git repository.
/// The values indicates the date (formatted) when the git repository was `init` by gtm.
/// Both keys and values are owned `PathBuf` and `String` respectively.
pub struct Projects(BTreeMap<PathBuf, String>);

impl Projects {
    pub fn config_path() -> Option<PathBuf> {
        let mut path = dirs::home_dir()?;
        path.push(".git-time-metric");
        path.push("project.json");
        Some(path)
    }

    pub fn config() -> Result<Self, io::Error> {
        let path = Self::config_path().unwrap();
        Self::from_file(&path)
    }

    pub fn from_file<P: AsRef<Path>>(filename: P) -> Result<Self, io::Error> {
        let file = File::open(filename)?;
        let map = serde_json::from_reader(file)?;
        Ok(Self(map))
    }

    /// Return how many projects are initialized.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn contains_project<P: AsRef<Path>>(&self, project_path: P) -> bool {
        self.0.contains_key(project_path.as_ref())
    }

    ///
    pub fn keys(&self) -> btree_map::Keys<'_, PathBuf, String> {
        self.0.keys()
    }
}

pub trait ProjectKey {
    fn key(&self) -> &str;
}

impl<P: AsRef<Path>> ProjectKey for P {
    fn key(&self) -> &str {
        self.as_ref().file_name().unwrap().to_str().unwrap()
    }
}

#[cfg(test)]
mod tests {
    // use super::InitProjects;
    // use maplit::btreemap;
    // use std::path::PathBuf;

    #[test]
    fn test() {
        // let projects = InitProjects(btreemap! {
        //     PathBuf::from("") => "".to_owned(),
        // });

        // for p in projects {}
        // println!("{:?}", projects.0);
    }
}
