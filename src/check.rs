use crate::{VCSError, VCSResult};
use std::path::Path;

/// Stores the options available for calling [`check_version_control`] and controls which checks if any are run
#[cfg_attr(feature = "clap", derive(clap::Args))]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CheckOptions {
    /// Does not return an error for dirty files nor generate the list of said files
    #[cfg_attr(feature = "clap", arg(long))]
    pub allow_dirty: bool,

    /// This option basically disables checking. If true no checks are done. (Not even if the `path` exists)
    #[cfg_attr(feature = "clap", arg(long))]
    pub allow_no_vcs: bool,

    /// Does not return an error for staged files nor generate the list of said files
    #[cfg_attr(feature = "clap", arg(long))]
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
    if !supporting_code::existing_vcs_repo(path.as_ref(), path.as_ref()) {
        return VCSResult::Err(VCSError::NoVCS);
    }

    if opts.allow_dirty && opts.allow_staged {
        return Ok(());
    }

    let mut dirty_files = Vec::new();
    let mut staged_files = Vec::new();
    if let Ok(repo) = git2::Repository::discover(path.as_ref()) {
        let mut repo_opts = git2::StatusOptions::new();
        repo_opts.include_ignored(false);
        repo_opts.include_untracked(true);
        for status in repo.statuses(Some(&mut repo_opts))?.iter() {
            if let Some(path) = status.path() {
                match status.status() {
                    git2::Status::CURRENT => (),
                    git2::Status::INDEX_NEW
                    | git2::Status::INDEX_MODIFIED
                    | git2::Status::INDEX_DELETED
                    | git2::Status::INDEX_RENAMED
                    | git2::Status::INDEX_TYPECHANGE => {
                        if !opts.allow_staged {
                            staged_files.push(path.to_string())
                        }
                    }
                    _ => {
                        if !opts.allow_dirty {
                            dirty_files.push(path.to_string())
                        }
                    }
                };
            }
        }
    }

    if dirty_files.is_empty() && staged_files.is_empty() {
        Ok(())
    } else {
        VCSResult::Err(VCSError::NotAllowedFilesFound {
            dirty_files,
            staged_files,
        })
    }
}

mod supporting_code;
