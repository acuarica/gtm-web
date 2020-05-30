use git2::Repository;
use std::path::{PathBuf, Path};

pub fn clone_repo<P: AsRef<Path>>(url: &str, into: P) -> Result<Repository, git2::Error> {
    let mut path = PathBuf::new();
    path.push(into);
    path.push("git-clone-repo");
    let repo = Repository::clone(url, path)?;
    Ok(repo)
}
