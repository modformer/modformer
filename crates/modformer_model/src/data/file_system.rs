use std::sync::RwLockReadGuard;

use anyhow::{
    Error,
    Result,
};

use super::directory::{
    Directory,
    DirectoryStrong,
};

const ROOT_DIR_NAME: &str = "<ROOT>";

#[derive(Debug)]
pub struct FileSystem<'a> {
    root: DirectoryStrong<'a>,
}

impl<'a> FileSystem<'a> {
    pub fn new() -> Self {
        Self {
            root: Directory::strong(ROOT_DIR_NAME, None),
        }
    }
}

impl<'a> FileSystem<'a> {
    pub(crate) fn read_root<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce(RwLockReadGuard<'_, Directory<'a>>) -> Result<()>,
    {
        self.root
            .try_read()
            .map_err(|_| Error::msg("message"))
            .and_then(f)
    }
}
