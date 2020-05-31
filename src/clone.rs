use git2::Repository;
use std::path::{Path, PathBuf};

fn fetch_notes(repo: &Repository) -> Result<(), git2::Error> {
    let mut remote = repo.find_remote("origin")?;
    remote.fetch(&["refs/notes/gtm-data:refs/notes/gtm-data"], None, None)?;
    Ok(())
}

pub fn clone_repo<P: AsRef<Path>>(url: &str, into: P) -> Result<Repository, git2::Error> {
    let mut path = PathBuf::new();
    path.push(into);
    path.push("git-clone-repo");
    let repo = Repository::clone(url, path)?;
    fetch_notes(&repo)?;
    Ok(repo)
}

#[cfg(test)]
mod tests {

    use super::clone_repo;
    use crate::{get_notes, NotesFilter};
    use std::error::Error;
    use tempfile::tempdir;

    #[test]
    fn test_clone() -> Result<(), Box<dyn Error>> {
        let tempdir = tempdir()?;
        let path = tempdir.path();
        let url = "https://github.com/acuarica/gtm-web.git";

        println!("Cloning remote `{}` into `{:?}`", url, path);
        let repo = clone_repo(url, path)?;
        let mut notes = Vec::new();
        get_notes(
            |c| {
                println!("{:?}", c.commit);
                notes.push(c.commit);
            },
            &repo,
            "sdfsdf".to_owned(),
            &NotesFilter::no_filter(),
        )?;
        assert_ne!(notes.len(), 0);

        Ok(())
    }
}
