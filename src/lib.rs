#![warn(missing_docs)]

//! [![Crates.io](https://img.shields.io/crates/v/file_type_enum.svg)](https://crates.io/crates/file_type_enum)
//! [![Docs.rs](https://docs.rs/file_type_enum/badge.svg)](https://docs.rs/file_type_enum)
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/marcospb19/file_type_enum/blob/main/LICENSE)
//!
//! An enum with a variant for each file type.
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
//! If you don't need an enum, check these methods from `std` instead:
//!
//! - [`Path::is_file`].
//! - [`Path::is_dir`].
//! - [`Path::is_symlink`].
//!
//! # Example
//!
//! ```
//! use file_type_enum::FileType;
//! use std::io;
//!
//! fn main() -> io::Result<()> {
//!     let file_type = FileType::from_path("/tmp")?;
//!
//!     println!("There's a {} at /tmp", file_type);
//!     // Out:  "There's a directory at /tmp"
//!
//!     Ok(())
//! }
//! ```
//!
//! Note that the [`FileType::from_path`] follows symlinks and [`FileType::from_symlink_path`] does not.
//!
//! # Conversions
//!
//! - From [`AsRef<Path>`], [`fs::Metadata`] and [std's `FileType`].
//! - From and into [`libc::mode_t`] (via the feature `"mode-t-conversion"`).
//!
//! # Contributing
//!
//! Issues and PRs are welcome.
//!
//! [`AsRef<Path>`]: std::path::Path
//! [`fs::Metadata`]: std::fs::Metadata
//! [std's `FileType`]: std::fs::FileType

#[cfg(feature = "mode-t-conversion")]
mod mode_t_conversion_feature;

#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;
use std::{fmt, fs, io, path::Path};

#[cfg(feature = "mode-t-conversion")]
pub use mode_t_conversion_feature::*;

/// An enum with a variant for each file type.
///
/// ```
/// # use file_type_enum::FileType;
/// # let file_type = FileType::from_path("src/").unwrap();
/// match file_type {
///     FileType::Regular     => {},
///     FileType::Directory   => {},
///     FileType::Symlink     => {},
///     FileType::BlockDevice => {}, // unix only
///     FileType::CharDevice  => {}, // unix only
///     FileType::Fifo        => {}, // unix only
///     FileType::Socket      => {}, // unix only
/// }
/// ```
#[rustfmt::skip]
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
pub enum FileType {
    /// A regular file (e.g. '.txt', '.rs', '.zip').
    Regular,
    /// A directory, folder of files.
    Directory,
    /// A symbolic link, points to another path.
    Symlink,
    /// Unix block device.
    #[cfg(unix)] BlockDevice,
    /// Unix char device.
    #[cfg(unix)] CharDevice,
    /// Unix FIFO.
    #[cfg(unix)] Fifo,
    /// Unix socket.
    #[cfg(unix)] Socket,
}

impl FileType {
    /// Reads a `FileType` from a path.
    ///
    /// This function follows symlinks, so it can never return a `FileType::Symlink`.
    ///
    /// # Example
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
    /// # Errors
    ///
    /// - Path does not exist, or
    /// - Current user lacks permissions to read `fs::Metadata` of `path`.
    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let fs_file_type = fs::metadata(path.as_ref())?.file_type();
        let result = FileType::from(fs_file_type);
        Ok(result)
    }

    /// Reads a `FileType` from a path, considers symlinks.
    ///
    /// This function does not follow symlinks, so the result can be the variant `FileType::Symlink` too, unlike [`FileType::from_path`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use file_type_enum::FileType;
    ///
    /// let path = "/dev/stdout";
    /// let file_type = FileType::from_symlink_path(path).unwrap();
    ///
    /// println!("There's a {file_type} at {path}");
    /// // Out:  "There's a symlink     at /dev/stdout"
    /// ```
    ///
    /// # Errors
    ///
    /// - Path does not exist, or
    /// - Current user lacks permissions to read `fs::Metadata` of `path`.
    pub fn from_symlink_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let fs_file_type = fs::symlink_metadata(path.as_ref())?.file_type();
        let result = FileType::from(fs_file_type);
        Ok(result)
    }

    /// Returns true if is a [`FileType::Regular`].
    pub fn is_regular(&self) -> bool {
        matches!(self, FileType::Regular)
    }

    /// Returns true if is a [`FileType::Directory`].
    pub fn is_directory(&self) -> bool {
        matches!(self, FileType::Directory)
    }

    /// Returns true if is a [`FileType::Symlink`].
    pub fn is_symlink(&self) -> bool {
        matches!(self, FileType::Symlink)
    }

    /// Returns true if is a [`FileType::BlockDevice`].
    #[cfg(unix)]
    pub fn is_block_device(&self) -> bool {
        matches!(self, FileType::BlockDevice)
    }

    /// Returns true if is a [`FileType::CharDevice`].
    #[cfg(unix)]
    pub fn is_char_device(&self) -> bool {
        matches!(self, FileType::CharDevice)
    }

    /// Returns true if is a [`FileType::Fifo`].
    #[cfg(unix)]
    pub fn is_fifo(&self) -> bool {
        matches!(self, FileType::Fifo)
    }

    /// Returns true if is a [`FileType::Socket`].
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
}
