use std::{
    collections::HashMap,
    sync::{
        Arc,
        RwLock,
    },
};

use super::{
    directory::{
        DirectoryStrong,
        DirectoryWeak,
    },
    file::FileStrong,
};

#[derive(Debug)]
pub enum Parent<'a> {
    Directory(DirectoryWeak<'a>),
}

#[derive(Debug)]
pub enum Child<'a> {
    Directory(DirectoryStrong<'a>),
    File(FileStrong<'a>),
}

pub type Children<'a> = HashMap<&'a str, Child<'a>>;
pub type ChildrenStrong<'a> = Arc<RwLock<Children<'a>>>;
