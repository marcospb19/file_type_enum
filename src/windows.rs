use std::{fmt, fs, io, path::Path};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
pub enum FileType {
    File,
    Directory,
    Symlink,
}

impl FileType {
    pub fn try_from_path(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let ft: fs::FileType = fs::metadata(path.as_ref())?.file_type();
        // Check each type, except for symlink, because fs::metadata() follows symlinks
        let result = {
            if ft.is_file() {
                FileType::File
            } else if ft.is_dir() {
                FileType::Directory
            } else {
                panic!();
            }
        };
        Ok(result)
    }

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
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileType::File => write!(f, "regular file"),
            FileType::Directory => write!(f, "directory"),
            FileType::Symlink => write!(f, "symbolic link"),
        }
    }
}
