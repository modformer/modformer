use std::{
    collections::HashMap,
    sync::Arc,
};

use tokio::sync::RwLock;

use super::{
    directory::{
        StrongDirectory,
        WeakDirectory,
    },
    file::StrongFile,
};

#[derive(Debug)]
pub enum Parent<'a> {
    Directory(WeakDirectory<'a>),
}

#[derive(Debug)]
pub enum Child<'a> {
    Directory(StrongDirectory<'a>),
    File(StrongFile<'a>),
}

pub type Children<'a> = HashMap<&'a str, Child<'a>>;
pub type StrongChildren<'a> = Arc<RwLock<Children<'a>>>;
