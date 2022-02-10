#![forbid(unsafe_code)]

use anyhow::Result;
use async_trait::async_trait;
use modformer::{
    data::FileSystem,
    process::Read,
};

#[derive(Debug)]
pub struct Fs {}

impl Fs {
    pub fn new() -> Self {
        Fs {}
    }
}

#[async_trait]
impl Read for Fs {
    async fn read(&self, _fs: &mut FileSystem) -> Result<()> {
        Ok(())
    }
}
