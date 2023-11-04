use crate::VCSResult;
use cargo_util::ProcessBuilder;
use std::path::Path;

// Check if we are in an existing repo. We define that to be true if either:
//
// 1. We are in a git repo and the path to the new package is not an ignored
//    path in that repo.
// 2. We are in an HG repo.
pub fn existing_vcs_repo(path: &Path, cwd: &Path) -> bool {
    fn in_git_repo(path: &Path, cwd: &Path) -> bool {
        if let Ok(repo) = GitRepo::discover(path, cwd) {
            // Don't check if the working directory itself is ignored.
            if repo.workdir().map_or(false, |workdir| workdir == path) {
                true
            } else {
                !repo.is_path_ignored(path).unwrap_or(false)
            }
        } else {
            false
        }
    }

    in_git_repo(path, cwd) || HgRepo::discover(path, cwd).is_ok()
}

pub struct HgRepo;

pub struct GitRepo;

impl GitRepo {
    pub fn discover(path: &Path, _: &Path) -> Result<git2::Repository, git2::Error> {
        git2::Repository::discover(path)
    }
}

impl HgRepo {
    pub fn discover(path: &Path, cwd: &Path) -> VCSResult<HgRepo> {
        ProcessBuilder::new("hg")
            .cwd(cwd)
            .arg("--cwd")
            .arg(path)
            .arg("root")
            .exec_with_output()?;
        Ok(HgRepo)
    }
}
