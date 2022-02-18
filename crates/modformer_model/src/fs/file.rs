use std::sync::Arc;

use tokio::sync::RwLock;

use super::relationships::{
    Child,
    Parent,
};

// File

#[derive(Debug)]
pub struct File<'a> {
    _name: &'a str,
    _content: String,
    _parent: Parent<'a>,
}

pub type StrongFile<'a> = Arc<RwLock<File<'a>>>;

impl<'a> Into<Child<'a>> for StrongFile<'a> {
    fn into(self) -> Child<'a> {
        Child::File(self)
    }
}

impl<'a> File<'a> {
    pub fn new<N>(name: N, content: String, parent: Parent<'a>) -> Self
    where
        N: Into<&'a str>,
    {
        Self {
            _name: name.into(),
            _content: content,
            _parent: parent,
        }
    }

    pub fn strong<N>(name: N, content: String, parent: Parent<'a>) -> StrongFile<'a>
    where
        N: Into<&'a str>,
    {
        Arc::new_cyclic(|_| RwLock::new(File::new(name.into(), content, parent)))
    }
}
