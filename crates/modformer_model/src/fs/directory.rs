use std::sync::{
    Arc,
    Weak,
};

use anyhow::{
    Error,
    Result,
};
use tokio::sync::RwLock;

use super::{
    file::File,
    file_system::FileSystem,
    relationships::{
        Child,
        Parent,
        StrongChildren,
    },
};

#[derive(Debug)]
pub struct Directory<'a> {
    children: StrongChildren<'a>,
    directory: WeakDirectory<'a>,
    name: &'a str,
    _parent: Option<Parent<'a>>,
}

pub type StrongDirectory<'a> = Arc<RwLock<Directory<'a>>>;
pub type WeakDirectory<'a> = Weak<RwLock<Directory<'a>>>;

impl<'a> Into<Child<'a>> for StrongDirectory<'a> {
    fn into(self) -> Child<'a> {
        Child::Directory(self)
    }
}

impl<'a> Directory<'a> {
    fn new<N>(name: N, directory: WeakDirectory<'a>, parent: Option<Parent<'a>>) -> Self
    where
        N: Into<&'a str>,
    {
        Self {
            children: Default::default(),
            directory,
            name: name.into(),
            _parent: parent,
        }
    }

    pub fn strong<N>(name: N, parent: Option<Parent<'a>>) -> StrongDirectory<'a>
    where
        N: Into<&'a str>,
    {
        Arc::new_cyclic(|dir| RwLock::new(Directory::new(name, dir.clone(), parent)))
    }

    pub fn name(&self) -> &'a str {
        self.name
    }
}

impl<'a> Directory<'a> {
    async fn insert<C>(&self, name: &'a str, child: C) -> Result<()>
    where
        C: Into<Child<'a>>,
    {
        let mut children = self.children.write().await;
        let child = child.into();
        let res = children
            .try_insert(name, child)
            .map(|_| ())
            .map_err(|_| Error::msg(""));

        res
    }
}

impl<'a> Directory<'a> {
    pub async fn create_dir<N>(file_system: &FileSystem<'a>, name: N) -> Result<()>
    where
        N: Into<&'a str>,
    {
        let dir = file_system.root.read().await;
        let res = dir.create_dir_internal(name).await;

        res
    }

    async fn create_dir_internal<N>(&self, name: N) -> Result<()>
    where
        N: Into<&'a str>,
    {
        let name = name.into();
        let parent = Parent::Directory(self.directory.clone());
        let dir = Directory::strong(name, Some(parent));
        let res = self.insert(name, dir).await;

        res
    }
}

impl<'a> Directory<'a> {
    pub async fn create_file<N>(
        file_system: &FileSystem<'a>,
        name: N,
        content: String,
    ) -> Result<()>
    where
        N: Into<&'a str>,
    {
        let dir = file_system.root.read().await;
        let res = dir.create_file_internal(name, content).await;

        res
    }

    async fn create_file_internal<N>(&self, name: N, content: String) -> Result<()>
    where
        N: Into<&'a str>,
    {
        let name = name.into();
        let parent = Parent::Directory(self.directory.clone());
        let file = File::strong(name, content, parent);
        let res = self.insert(name, file).await;

        res
    }
}
