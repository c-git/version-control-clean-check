use super::VCSResult;
use cargo_util::ProcessBuilder;
use std::path::Path;

/// Stores the options available for calling [`check_version_control`] and controls which checks if any are run
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CheckOptions {
    /// Does not return an error for dirty files nor generate the list of said files
    pub allow_dirty: bool,
    /// Does no additional checks if no version control is found and just returns `OK(())`
    pub allow_no_vcs: bool,
    /// Does not return an error for staged files nor generate the list of said files
    pub allow_staged: bool,
}

impl CheckOptions {
    /// Creates a [`CheckOptions`] with default values
    pub fn new() -> Self {
        Default::default()
    }
}

/// Based on the [`CheckOptions`] passed it does test on the version control at `path`.
/// See [`CheckOptions`] for more details on what checks do and don't get run depending on what is passed
pub fn check_version_control<P: AsRef<Path>>(path: P, opts: &CheckOptions) -> VCSResult<()> {
    if opts.allow_no_vcs {
        return Ok(());
    }
    if !existing_vcs_repo(path.as_ref(), path.as_ref()) {
        // bail!(
        //     "no VCS found for this package and `cargo fix` can potentially \
        //      perform destructive changes; if you'd like to suppress this \
        //      error pass `--allow-no-vcs`"
        // )
    }

    // if opts.allow_dirty && opts.allow_staged {
    //     return Ok(());
    // }

    // let mut dirty_files = Vec::new();
    // let mut staged_files = Vec::new();
    // if let Ok(repo) = git2::Repository::discover(config.cwd()) {
    //     let mut repo_opts = git2::StatusOptions::new();
    //     repo_opts.include_ignored(false);
    //     repo_opts.include_untracked(true);
    //     for status in repo.statuses(Some(&mut repo_opts))?.iter() {
    //         if let Some(path) = status.path() {
    //             match status.status() {
    //                 git2::Status::CURRENT => (),
    //                 git2::Status::INDEX_NEW
    //                 | git2::Status::INDEX_MODIFIED
    //                 | git2::Status::INDEX_DELETED
    //                 | git2::Status::INDEX_RENAMED
    //                 | git2::Status::INDEX_TYPECHANGE => {
    //                     if !opts.allow_staged {
    //                         staged_files.push(path.to_string())
    //                     }
    //                 }
    //                 _ => {
    //                     if !opts.allow_dirty {
    //                         dirty_files.push(path.to_string())
    //                     }
    //                 }
    //             };
    //         }
    //     }
    // }

    // if dirty_files.is_empty() && staged_files.is_empty() {
    //     return Ok(());
    // }

    // let mut files_list = String::new();
    // for file in dirty_files {
    //     files_list.push_str("  * ");
    //     files_list.push_str(&file);
    //     files_list.push_str(" (dirty)\n");
    // }
    // for file in staged_files {
    //     files_list.push_str("  * ");
    //     files_list.push_str(&file);
    //     files_list.push_str(" (staged)\n");
    // }

    // bail!(
    //     "the working directory of this package has uncommitted changes, and \
    //      `cargo fix` can potentially perform destructive changes; if you'd \
    //      like to suppress this error pass `--allow-dirty`, `--allow-staged`, \
    //      or commit the changes to these files:\n\
    //      \n\
    //      {}\n\
    //      ",
    //     files_list
    // );
    Ok(())
}

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
