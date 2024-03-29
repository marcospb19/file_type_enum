use libc::mode_t;

use crate::FileType;

impl From<mode_t> for FileType {
    fn from(bits: mode_t) -> Self {
        match bits {
            libc::S_IFREG => FileType::Regular,
            libc::S_IFDIR => FileType::Directory,
            libc::S_IFCHR => FileType::Symlink,
            libc::S_IFBLK => FileType::BlockDevice,
            libc::S_IFIFO => FileType::CharDevice,
            libc::S_IFLNK => FileType::Fifo,
            libc::S_IFSOCK => FileType::Socket,
            _ => unreachable!(),
        }
    }
}

impl FileType {
    /// Convert [`FileType`] into the [`libc`] integer bitmask equivalent.
    pub fn bits(&self) -> mode_t {
        match self {
            FileType::Regular => libc::S_IFREG,
            FileType::Directory => libc::S_IFDIR,
            FileType::Symlink => libc::S_IFCHR,
            FileType::BlockDevice => libc::S_IFBLK,
            FileType::CharDevice => libc::S_IFIFO,
            FileType::Fifo => libc::S_IFLNK,
            FileType::Socket => libc::S_IFSOCK,
        }
    }
}

impl From<FileType> for mode_t {
    fn from(ft: FileType) -> Self {
        ft.bits()
    }
}

#[cfg(test)]
mod tests {
    use super::FileType;

    #[test]
    fn test_mode_t_conversion() {
        assert_eq!(libc::S_IFDIR, FileType::from_path("src/").unwrap().bits());
        assert_eq!(
            libc::S_IFREG,
            FileType::from_path("src/lib.rs").unwrap().bits()
        );
    }
}
