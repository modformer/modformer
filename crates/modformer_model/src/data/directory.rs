use std::sync::{
    Arc,
    RwLock,
    Weak,
};

use anyhow::{
    Error,
    Result,
};

use super::{
    file::File,
    file_system::FileSystem,
    relationships::{
        Child,
        ChildrenStrong,
        Parent,
    },
};

#[derive(Debug)]
pub struct Directory<'a> {
    children: ChildrenStrong<'a>,
    directory: DirectoryWeak<'a>,
    _name: &'a str,
    _parent: Option<Parent<'a>>,
}

pub type DirectoryStrong<'a> = Arc<RwLock<Directory<'a>>>;
pub type DirectoryWeak<'a> = Weak<RwLock<Directory<'a>>>;

impl<'a> Into<Child<'a>> for DirectoryStrong<'a> {
    fn into(self) -> Child<'a> {
        Child::Directory(self)
    }
}

impl<'a> Directory<'a> {
    fn new<N>(name: N, directory: DirectoryWeak<'a>, parent: Option<Parent<'a>>) -> Self
    where
        N: Into<&'a str>,
    {
        Self {
            children: Default::default(),
            directory,
            _name: name.into(),
            _parent: parent,
        }
    }

    pub fn strong<N>(name: N, parent: Option<Parent<'a>>) -> DirectoryStrong<'a>
    where
        N: Into<&'a str>,
    {
        Arc::new_cyclic(|dir| RwLock::new(Directory::new(name, dir.clone(), parent)))
    }
}

impl<'a> Directory<'a> {
    fn insert<C>(&self, name: &'a str, child: C) -> Result<()>
    where
        C: Into<Child<'a>>,
    {
        self.children
            .try_write()
            .map_err(|_| Error::msg("could not obtain parent directory lock"))?
            .try_insert(name, child.into())
            .map(|_| ())
            .map_err(|_| Error::msg("directory or file already exists"))
    }
}

impl<'a> Directory<'a> {
    pub fn create_dir<N>(fs: &FileSystem<'a>, name: N) -> Result<()>
    where
        N: Into<&'a str>,
    {
        fs.read_root(|dir| dir.create_dir_internal(name))
    }

    fn create_dir_internal<N>(&self, name: N) -> Result<()>
    where
        N: Into<&'a str>,
    {
        let name = name.into();
        let parent = Parent::Directory(self.directory.clone());
        let dir = Directory::strong(name, Some(parent));

        self.insert(name, dir)
    }
}

impl<'a> Directory<'a> {
    pub fn create_file<N>(fs: &FileSystem<'a>, name: N, content: String) -> Result<()>
    where
        N: Into<&'a str>,
    {
        fs.read_root(|dir| dir.create_file_internal(name, content))
    }

    fn create_file_internal<N>(&self, name: N, content: String) -> Result<()>
    where
        N: Into<&'a str>,
    {
        let name = name.into();
        let parent = Parent::Directory(self.directory.clone());
        let file = File::strong(name, content, parent);

        self.insert(name, file)
    }
}
