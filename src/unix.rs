use std::{fmt, fs, io, os::unix::fs::FileTypeExt, path::Path};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
pub enum FileType {
    File,
    Directory,
    Symlink,
    BlockDevice,
    CharDevice,
    Fifo,
    Socket,
}

/// # Variants:
/// ```rust
/// match file_type {
///     FileType::File        => { /* ... */ },
///     FileType::Directory   => { /* ... */ },
///     FileType::Symlink     => { /* ... */ },
///     FileType::BlockDevice => { /* ... */ },
///     FileType::CharDevice  => { /* ... */ },
///     FileType::Fifo        => { /* ... */ },
///     FileType::Socket      => { /* ... */ },
/// }
/// ```
impl FileType {
    /// Try to get `FileType` from a path.
    ///
    /// This function follows symlinks, so it cannot ever return a
    /// FileType::Symlink, see `try_from_symlink_path` if you wanna check
    /// symlinks.
    pub fn try_from_path(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let ft: fs::FileType = fs::metadata(path.as_ref())?.file_type();
        // Check each type, except for symlink, because fs::metadata() follows symlinks
        let result = {
            if ft.is_file() {
                FileType::File
            } else if ft.is_dir() {
                FileType::Directory
            } else if ft.is_block_device() {
                FileType::BlockDevice
            } else if ft.is_char_device() {
                FileType::CharDevice
            } else if ft.is_fifo() {
                FileType::Fifo
            } else if ft.is_socket() {
                FileType::Socket
            } else {
                panic!();
            }
        };
        Ok(result)
    }

    /// Try to get `FileType` from a path.
    ///
    /// Don't follow symlinks, so the result can be `FileType::Symlink` itself.
    pub fn try_from_symlink_path(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let ft: fs::FileType = fs::symlink_metadata(path.as_ref())?.file_type();
        // Check each type
        let result = {
            if ft.is_file() {
                FileType::File
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
                panic!();
            }
        };

        Ok(result)
    }

    pub fn is_file(&self) -> bool {
        match self {
            FileType::File => true,
            _ => false,
        }
    }

    pub fn is_directory(&self) -> bool {
        match self {
            FileType::Directory => true,
            _ => false,
        }
    }

    pub fn is_symlink(&self) -> bool {
        match self {
            FileType::Symlink => true,
            _ => false,
        }
    }

    pub fn is_block_device(&self) -> bool {
        match self {
            FileType::BlockDevice => true,
            _ => false,
        }
    }

    pub fn is_char_device(&self) -> bool {
        match self {
            FileType::CharDevice => true,
            _ => false,
        }
    }

    pub fn is_fifo(&self) -> bool {
        match self {
            FileType::Fifo => true,
            _ => false,
        }
    }

    pub fn is_socket(&self) -> bool {
        match self {
            FileType::Socket => true,
            _ => false,
        }
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileType::File => write!(f, "regular file"),
            FileType::Directory => write!(f, "directory"),
            FileType::Symlink => write!(f, "symbolic link"),
            FileType::BlockDevice => write!(f, "block device"),
            FileType::CharDevice => write!(f, "char device"),
            FileType::Fifo => write!(f, "FIFO"),
            FileType::Socket => write!(f, "socket"),
        }
    }
}
