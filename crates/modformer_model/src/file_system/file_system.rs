use super::directory::{
    Directory,
    DirectoryStrong,
};

const ROOT_DIR_NAME: &str = "<ROOT>";

#[derive(Debug)]
pub struct FileSystem<'a> {
    pub(crate) root: DirectoryStrong<'a>,
}

impl<'a> FileSystem<'a> {
    pub fn new() -> Self {
        Self {
            root: Directory::strong(ROOT_DIR_NAME, None),
        }
    }
}
