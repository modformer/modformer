use std::slice::IterMut;

pub trait Entries {
    fn entries_iter(&mut self) -> IterMut<Entry>;
}

#[derive(Debug)]
pub struct Vfs {
    entries: Vec<Entry>,
}

impl Vfs {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl Entries for Vfs {
    fn entries_iter(&mut self) -> IterMut<Entry> {
        self.entries.iter_mut()
    }
}

#[derive(Debug)]
pub enum Entry {
    Directory(Directory),
    File(File),
}

#[derive(Debug)]
pub struct Directory {
    entries: Vec<Entry>,
    _name: String,
}

impl Directory {
    pub fn new<N>(name: N) -> Self
    where
        N: Into<String>,
    {
        Self {
            entries: Vec::new(),
            _name: name.into(),
        }
    }
}

impl Entries for Directory {
    fn entries_iter(&mut self) -> IterMut<Entry> {
        self.entries.iter_mut()
    }
}

#[derive(Debug)]
pub struct File {
    _content: Content,
    _name: String,
}

impl File {
    pub fn new<N>(name: N, content: Content) -> Self
    where
        N: Into<String>,
    {
        Self {
            _content: content,
            _name: name.into(),
        }
    }
}

#[derive(Debug)]
pub enum Content {
    Binary(Vec<u8>),
    Text(String),
}
