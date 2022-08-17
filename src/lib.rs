//! [![Crates.io](https://img.shields.io/crates/v/file_type_enum.svg)](https://crates.io/crates/file_type_enum)
//! [![Docs.rs](https://docs.rs/file_type_enum/badge.svg)](https://docs.rs/file_type_enum)
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/marcospb19/file_type_enum/blob/main/LICENSE)
//!
//! A enum with one variant for each file type.
//!
//! # Enum [`FileType`]:
//!
//! ```rust
//! pub enum FileType {
//!     Regular,
//!     Directory,
//!     Symlink,
//!     BlockDevice, // unix only
//!     CharDevice,  // unix only
//!     Fifo,        // unix only
//!     Socket,      // unix only
//! }
//! ```
//!
//! # Example:
//!
//! ```rust
//! use file_type_enum::FileType;
//!
//! let path = "/tmp";
//! let file_type = FileType::from_path(path).unwrap();
//!
//! println!("There's a {} at {}!", file_type, path);
//! // Outputs: "There's a directory at /tmp!"
//! ```
//!
//! # Errors:
//!
//! - If path does not exist, or
//! - Current user don't have permissions to read `fs::Metadata` from `path`.
//!
//! ---
//!
//! For each variant, there is also a short hand method:
//!
//! ```ignore
//! let ft = FileType::from(path);
//! if ft.is_regular() { ... }
//! if ft.is_directory() { ... }
//! if ft.is_symlink() { ... }
//! if ft.is_block_device() { ... }
//! if ft.is_char_device() { ... }
//! if ft.is_fifo() { ... }
//! if ft.is_socket() { ... }
//! ```
//!
//! ```rust
//! use file_type_enum::FileType;
//!
//! let path = ".git";
//! let file_type = FileType::from_path(path).unwrap();
//!
//! if file_type.is_directory() {
//!     println!("We are at the root of a git repository.");
//! }
//! ```
//!
//! ---
//!
//! If `path` points to a _symlink_, `from_path(path)` follows it, so the
//! returned type can never be a _symlink_.
//!
//! To avoid this, use `FileType::from_symlink_path`, this don't follow, and can
//! return a _symlink_.
//!
//! ```rust
//! use file_type_enum::FileType;
//!
//! let path = "/dev/stdout";
//! let file_type = FileType::from_symlink_path(path).unwrap();
//!
//! println!("There's a {} at {}!", file_type, path);
//! // Outputs: "There's a symbolic link at /dev/stdout!"
//! ```
//!
//! ---
//!
//! # Conversions
//!
//! - From `std::fs::FileType`.
//! - From and into `libc::mode_t` (enable the `mode-t-conversion` optional
//!   feature).
//!
//! # Helping and contributing:
//!
//! It's easy to contribute to this crate, here are some options:
//! - Share it to a friend.
//! - Help improve this README.md, even with little details.
//! - Open an issue or PR in the repository.
//! - Leave a star on GitHub.
//! - Use it!!!

#[cfg(feature = "mode-t-conversion")]
mod mode_t_conversion_feature;

#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;
use std::{fmt, fs, io, path::Path};

#[cfg(feature = "mode-t-conversion")]
pub use mode_t_conversion_feature::*;

/// An enum with a variant for each file type.
///
/// ```ignore
/// match file_type {
///     FileType::Regular     => { /* ... */ },
///     FileType::Directory   => { /* ... */ },
///     FileType::Symlink     => { /* ... */ },
///     FileType::BlockDevice => { /* ... */ }, // unix only
///     FileType::CharDevice  => { /* ... */ }, // unix only
///     FileType::Fifo        => { /* ... */ }, // unix only
///     FileType::Socket      => { /* ... */ }, // unix only
/// }
/// ```
#[rustfmt::skip]
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
    #[cfg(unix)] BlockDevice,
    #[cfg(unix)] CharDevice,
    #[cfg(unix)] Fifo,
    #[cfg(unix)] Socket,
}

impl FileType {
    /// Try to get `FileType` from a path.
    ///
    /// This function follows symlinks, so it can never return a
    /// `FileType::Symlink`.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use file_type_enum::FileType;
    /// use std::io;
    ///
    /// fn main() -> io::Result<()> {
    ///     let is_everything_alright = FileType::from_path("/dev/tty")?.is_char_device();
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors:
    /// - Path does not exist.
    /// - The user lacks permissions to run `fs::metadata(path)`.
    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let fs_file_type = fs::metadata(path.as_ref())?.file_type();
        let result = FileType::from(fs_file_type);
        Ok(result)
    }

    /// Try to get `FileType` from a path.
    ///
    /// Don't follow symlinks, so the result can be the variant
    /// `FileType::Symlink` too.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use file_type_enum::FileType;
    ///
    /// let path = "/dev/stdout";
    /// let file_type = FileType::from_symlink_path(path).unwrap();
    ///
    /// println!("There's a {} at {}!", file_type, path);
    /// // Outputs: "There's a symlink at /dev/stdout!"
    /// ```
    ///
    /// # Errors:
    /// - Path does not exist.
    /// - The user lacks permissions to run `fs::symlink_metadata(path)`.
    pub fn from_symlink_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let fs_file_type = fs::symlink_metadata(path.as_ref())?.file_type();
        let result = FileType::from(fs_file_type);
        Ok(result)
    }

    pub fn is_regular(&self) -> bool {
        matches!(self, FileType::Regular)
    }

    pub fn is_directory(&self) -> bool {
        matches!(self, FileType::Directory)
    }

    pub fn is_symlink(&self) -> bool {
        matches!(self, FileType::Symlink)
    }

    #[cfg(unix)]
    pub fn is_block_device(&self) -> bool {
        matches!(self, FileType::BlockDevice)
    }

    #[cfg(unix)]
    pub fn is_char_device(&self) -> bool {
        matches!(self, FileType::CharDevice)
    }

    #[cfg(unix)]
    pub fn is_fifo(&self) -> bool {
        matches!(self, FileType::Fifo)
    }

    #[cfg(unix)]
    pub fn is_socket(&self) -> bool {
        matches!(self, FileType::Socket)
    }
}

impl From<fs::FileType> for FileType {
    fn from(ft: fs::FileType) -> Self {
        // Check each type
        #[cfg(unix)]
        let result = {
            if ft.is_file() {
                FileType::Regular
            } else if ft.is_dir() {
                FileType::Directory
            } else if ft.is_symlink() {
                FileType::Symlink
            } else if ft.is_block_device() {
                FileType::BlockDevice
            } else if ft.is_char_device() {
                FileType::CharDevice
            } else if ft.is_fifo() {
                FileType::Fifo
            } else if ft.is_socket() {
                FileType::Socket
            } else {
                unreachable!("file_type_enum: unexpected file type: {:?}.", ft)
            }
        };

        #[cfg(not(unix))]
        let result = {
            if ft.is_file() {
                FileType::Regular
            } else if ft.is_dir() {
                FileType::Directory
            } else if ft.is_symlink() {
                FileType::Symlink
            } else {
                unreachable!("file_type_enum: unexpected file type: {:?}.", ft)
            }
        };

        result
    }
}

impl From<fs::Metadata> for FileType {
    fn from(metadata: fs::Metadata) -> Self {
        metadata.file_type().into()
    }
}

impl fmt::Display for FileType {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileType::Regular => write!(f, "regular file"),
            FileType::Directory => write!(f, "directory"),
            FileType::Symlink => write!(f, "symbolic link"),
            #[cfg(unix)] FileType::BlockDevice => write!(f, "block device"),
            #[cfg(unix)] FileType::CharDevice => write!(f, "char device"),
            #[cfg(unix)] FileType::Fifo => write!(f, "FIFO"),
            #[cfg(unix)] FileType::Socket => write!(f, "socket"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FileType;

    #[test]
    fn test_with_this_repository_structured() {
        let this_file = FileType::from_path("src/lib.rs").unwrap();
        assert!(this_file.is_regular());
    }

    #[cfg(feature = "mode-t-conversion")]
    #[test]
    fn test_mode_t_conversion() {
        assert_eq!(libc::S_IFDIR, FileType::from_path("src/").unwrap().bits());
        assert_eq!(
            libc::S_IFREG,
            FileType::from_path("src/lib.rs").unwrap().bits()
        );
    }
}
