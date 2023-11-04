use std::{fmt::Debug, path::PathBuf};

use anyhow::bail;
use rstest::rstest;
use version_control_clean_check::{check_version_control, CheckOptions, VCSError, VCSResult};
use TestDir as TD;

#[derive(Debug)]
struct TestError(VCSError);
impl From<VCSError> for TestError {
    fn from(value: VCSError) -> Self {
        Self(value)
    }
}

impl<T: Debug> TryFrom<VCSResult<T>> for TestError {
    type Error = anyhow::Error;

    fn try_from(value: VCSResult<T>) -> Result<Self, Self::Error> {
        if value.is_ok() {
            bail!("Value is not error. Found: {value:?}");
        }
        Ok(Self(value.unwrap_err()))
    }
}

impl PartialEq for TestError {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (VCSError::NoVCS, VCSError::NoVCS) => true,
            (
                VCSError::NotAllowedFilesFound {
                    dirty_files: l_dirty_files,
                    staged_files: l_staged_files,
                },
                VCSError::NotAllowedFilesFound {
                    dirty_files: r_dirty_files,
                    staged_files: r_staged_files,
                },
            ) => l_dirty_files == r_dirty_files && l_staged_files == r_staged_files,
            (VCSError::GitError(..), _) | (VCSError::Anyhow(..), _) => false, // Never equal if not one of our local errors during testing
            _ => core::mem::discriminant(&self.0) == core::mem::discriminant(&other.0),
        }
    }
}

enum TestDir {
    NoVCS,
    Clean,
    StagedOnly,
    DirtyOnly,
    StagedAndDirty,
}

impl TestDir {
    fn to_path(&self) -> PathBuf {
        let base_test_folder = PathBuf::from("tests/test_folders/");
        let sub_folder = match self {
            TestDir::NoVCS => "no_vcs",
            TestDir::Clean => "clean",
            TestDir::StagedOnly => "staged_only",
            TestDir::DirtyOnly => "dirty_only",
            TestDir::StagedAndDirty => "staged_and_dirty",
        };
        let result = base_test_folder.join(sub_folder);
        assert!(result.exists(), "Path not found: {result:?}");
        result.canonicalize().unwrap()
    }
}

fn match_results(actual: VCSResult<()>, expected: VCSResult<()>) {
    match (&actual, &expected) {
        (Ok(_), Ok(_)) => (),
        (Ok(_), Err(_)) | (Err(_), Ok(_)) => {
            panic!("Actual and Expected do not match./n actual:{actual:?}/n expected: {expected:?}")
        }
        (Err(..), Err(..)) => {
            let actual_error = actual.unwrap_err();
            let expected_error = expected.unwrap_err();
            println!("---\nActual Error:\n{actual_error}\n");
            println!("---\nExpected Error:\n{expected_error}\n---");
            assert_eq!(TestError(actual_error), TestError(expected_error))
        }
    }
}

#[test]
fn non_existent_folder() {
    let mut opts = CheckOptions::new();
    let non_existent_path = PathBuf::from("non_existent_path_bfEHgMV62y5S7LYn");
    assert!(!non_existent_path.exists());

    // Test is no vcs
    let actual = check_version_control(&non_existent_path, &opts);
    match_results(actual, Err(VCSError::NoVCS));

    // Test passes if no vcs allowed
    opts.allow_no_vcs = true;
    let actual = check_version_control(&non_existent_path, &opts);
    match_results(actual, Ok(()));
}

fn test_check_version_control(opts: CheckOptions, test_dir: TestDir, expected: VCSResult<()>) {
    let path = test_dir.to_path();
    println!("Opts: {opts:?} Path: {path:?}");
    let actual = check_version_control(path, &opts);
    match_results(actual, expected);
}

#[rstest]
#[case(TD::NoVCS)]
#[case(TD::Clean)]
#[case(TD::StagedOnly)]
#[case(TD::DirtyOnly)]
#[case(TD::StagedAndDirty)]
fn allow_no_vcs(#[case] test_dir: TestDir) {
    let mut opts = CheckOptions::new();
    opts.allow_no_vcs = true;
    let expected = Ok(());
    test_check_version_control(opts, test_dir, expected);
}
