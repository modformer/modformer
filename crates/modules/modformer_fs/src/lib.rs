use anyhow::Result;
use async_trait::async_trait;
use modformer::{
    data::Vfs,
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
    async fn read(&self, _vfs: &mut Vfs) -> Result<()> {
        Ok(())
    }
}
