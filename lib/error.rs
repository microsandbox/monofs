use std::{
    convert::Infallible,
    error::Error,
    fmt::{self, Display},
    io,
};

use ipldstore::ipld::cid::Cid;
use thiserror::Error;

use crate::filesystem::Utf8UnixPathSegment;
use microsandbox_utils::error::MicrosandboxUtilsError;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The result of a file system operation.
pub type FsResult<T> = Result<T, FsError>;

/// An error that occurred during a file system operation.
#[derive(pretty_error_debug::Debug, Error)]
pub enum FsError {
    /// Infallible error.
    #[error("Infallible error")]
    Infallible(#[from] core::convert::Infallible),

    /// Not a file.
    #[error("Not a file: {0:?}")]
    NotAFile(String),

    /// Not a directory.
    #[error("Not a directory: {0:?}")]
    NotADirectory(String),

    /// Not a symbolic CID link.
    #[error("Not a symbolic CID link: {0:?}")]
    NotASymCidLink(String),

    /// Not a symbolic path link.
    #[error("Not a symbolic path link: {0:?}")]
    NotASymPathLink(String),

    /// Path not found.
    #[error("Path not found: {0}")]
    PathNotFound(String),

    /// Custom error.
    #[error(transparent)]
    Custom(#[from] AnyError),

    // /// DID related error.
    // #[error("DID error: {0}")]
    // Did(#[from] monoutils_did_wk::DidError),
    /// IPLD Store error.
    #[error("IPLD Store error: {0}")]
    IpldStore(#[from] ipldstore::StoreError),

    /// Invalid deserialized OpenFlag value
    #[error("Invalid OpenFlag value: {0}")]
    InvalidOpenFlag(u8),

    /// Invalid deserialized EntityFlag value
    #[error("Invalid EntityFlag value: {0}")]
    InvalidEntityFlag(u8),

    /// Invalid deserialized PathFlag value
    #[error("Invalid PathFlag value: {0}")]
    InvalidPathFlag(u8),

    /// Invalid path component
    #[error("Invalid path component: {0}")]
    InvalidPathComponent(String),

    /// Invalid search path with root.
    #[error("Invalid search path: {0}")]
    InvalidSearchPath(String),

    /// Symbolic CID link not supported yet.
    #[error("Symbolic CID link not supported yet: path: {0:?}")]
    SymCidLinkNotSupportedYet(Vec<Utf8UnixPathSegment>),

    /// Invalid search path empty.
    #[error("Invalid search path empty")]
    InvalidSearchPathEmpty,

    /// Unable to load entity.
    #[error("Unable to load entity: {0}")]
    UnableToLoadEntity(Cid),

    /// CID error.
    #[error("CID error: {0}")]
    CidError(#[from] ipldstore::ipld::cid::Error),

    /// Path has root.
    #[error("Path has root: {0}")]
    PathHasRoot(String),

    /// Source is not a directory.
    #[error("Source is not a directory: {0}")]
    SourceIsNotADir(String),

    /// Target is not a directory.
    #[error("Target is not a directory: {0}")]
    TargetIsNotADir(String),

    /// Path already exists.
    #[error("Path already exists: {0}")]
    PathExists(String),

    /// Path is empty.
    #[error("Path is empty")]
    PathIsEmpty,

    /// Maximum follow depth reached.
    #[error("Maximum follow depth reached")]
    MaxFollowDepthReached,

    /// Broken symbolic CID link.
    #[error("Broken symbolic CID link: {0}")]
    BrokenSymCidLink(Cid),

    /// Invalid operation.
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    /// IO error.
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    /// An error that occurred during a database operation.
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Mount point is not empty
    #[error("Mount point is not empty: {0}")]
    MountPointNotEmpty(String),

    /// Mount operation failed
    #[error("Mount operation failed: {0}")]
    MountFailed(String),

    /// Unmount operation failed
    #[error("Unmount operation failed: {0}")]
    UnmountFailed(String),

    /// No available ports found in range
    #[error("No available ports found in range {host}:{start}-{end}")]
    NoAvailablePorts {
        /// The host to bind the port to
        host: String,

        /// The start port number
        start: u32,

        /// The end port number
        end: u32,
    },

    /// Supervisor error.
    #[error("Supervisor error: {0}")]
    SupervisorError(String),

    /// mfsrun binary not found at specified location
    #[error("mfsrun binary not found at {path} from {src}")]
    MfsrunBinaryNotFound {
        /// The path that was checked
        path: String,

        /// Where the path came from - either "environment variable" or "default path"
        src: String,
    },

    /// Maximum search depth reached while looking for MFS root
    #[error("Maximum search depth ({max_depth}) reached while looking for MFS root starting from {path}")]
    MaxMfsRootSearchDepthReached {
        /// The maximum depth allowed
        max_depth: u32,

        /// The path we started searching from
        path: String,
    },

    /// No MFS root found
    #[error("No MFS root found in path hierarchy starting from {0}")]
    NoMfsRootFound(String),

    /// An error that occurred when a migration error occurred
    #[error("migration error: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),

    /// An error that occurred when a CBOR decode error occurred
    #[error("CBOR decode error: {0}")]
    CborDecodeError(#[from] serde_ipld_dagcbor::DecodeError<Infallible>),

    /// Child IO must be piped
    #[error("Child IO must be piped")]
    ChildIoMustBePiped,
}

/// An error that can represent any error.
#[derive(Debug)]
pub struct AnyError {
    error: anyhow::Error,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl FsError {
    /// Creates a new `Err` result.
    pub fn custom(error: impl Into<anyhow::Error>) -> FsError {
        FsError::Custom(AnyError {
            error: error.into(),
        })
    }
}

impl AnyError {
    /// Downcasts the error to a `T`.
    pub fn downcast<T>(&self) -> Option<&T>
    where
        T: Display + fmt::Debug + Send + Sync + 'static,
    {
        self.error.downcast_ref::<T>()
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Creates an `Ok` `FsResult`.
#[allow(non_snake_case)]
pub fn Ok<T>(value: T) -> FsResult<T> {
    Result::Ok(value)
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl PartialEq for AnyError {
    fn eq(&self, other: &Self) -> bool {
        self.error.to_string() == other.error.to_string()
    }
}

impl Display for AnyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Error for AnyError {}

impl From<MicrosandboxUtilsError> for FsError {
    fn from(err: MicrosandboxUtilsError) -> Self {
        FsError::SupervisorError(err.to_string())
    }
}
