use std::fmt::Debug;

use anyhow::Result;
use async_trait::async_trait;

use crate::data::Vfs;

// Module

#[async_trait]
pub trait Read: Debug {
    async fn read(&self, vfs: &mut Vfs) -> Result<()>;
}

#[async_trait]
pub trait Transform: Debug {
    async fn transform(&self, vfs: &mut Vfs) -> Result<()>;
}

#[async_trait]
pub trait Write: Debug {
    async fn write(&self, vfs: &mut Vfs) -> Result<()>;
}
