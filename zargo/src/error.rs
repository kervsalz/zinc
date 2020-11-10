//!
//! The Zargo package manager error.
//!

use thiserror::Error;

///
/// The Zargo package manager `publish` subcommand error.
///
#[derive(Debug, Error)]
pub enum Error {
    /// The invalid project name error.
    #[error("project name is missing and cannot be inferred from path {0:?}")]
    ProjectNameInvalid(std::ffi::OsString),

    /// The invalid project type error.
    #[error("project type must be either `circuit` or `contract`, found `{0}`")]
    ProjectTypeInvalid(String),

    /// The project directory does not exist. Use `new` instead.
    #[error("directory {0:?} does not exist. To create a new directory, use `zargo new`")]
    DirectoryDoesNotExist(std::ffi::OsString),

    /// The project directory already exists. Use `init` instead.
    #[error("directory {0:?} already exists. To initialize it with a project, use `zargo init`")]
    DirectoryAlreadyExists(std::ffi::OsString),

    /// The project has been already initialized error.
    #[error("project at path {0:?} is already initialized")]
    ProjectAlreadyInitialized(std::ffi::OsString),

    /// The child process failure exit code.
    #[error("the subprocess failed with status {0}")]
    SubprocessFailure(std::process::ExitStatus),

    /// The child process stdin acquisition has failed.
    #[error("the subprocess stdin acquisition failed")]
    StdinAcquisition,

    /// The invalid network error.
    #[error("invalid network name: {0}")]
    NetworkInvalid(String),

    /// The unimplemented network error.
    #[error("unimplemented network: {0}")]
    NetworkUnimplemented(zksync::Network),

    /// The project is not a contract.
    #[error("not a contract")]
    NotAContract,

    /// The contract method to call is missing.
    #[error("contract method to call must be specified")]
    MethodMissing,

    /// The constructor arguments not found.
    #[error("constructor arguments not found")]
    ConstructorArgumentsNotFound,

    /// The input file data is invalid.
    #[error("invalid input file data")]
    InvalidInputData,

    /// The smart contract server failure.
    #[error("action failed: {0}")]
    ActionFailed(String),
}
