use std::path::Path;

use git2::Repository;

pub fn init<P: AsRef<Path>>(path: P) -> anyhow::Result<Repository> {
    Ok(git2::Repository::init(path)?)
}

pub fn add_all(repo: Repository, files: &[&str]) -> anyhow::Result<()> {
    let mut index = repo.index()?;
    index.add_all(files, git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    Ok(())
}

pub fn commit<P: AsRef<Path>>(path: P, msg: &str) -> anyhow::Result<()> {
    todo!()
}

pub fn commit_irrelevant_msg<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    commit(path, "irrelevant")
}
