use std::path::Path;

pub fn init<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    todo!()
}

pub fn add<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    todo!()
}

pub fn commit<P: AsRef<Path>>(path: P, msg: &str) -> anyhow::Result<()> {
    todo!()
}

pub fn commit_irrelevant_msg<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    commit(path, "irrelevant")
}
