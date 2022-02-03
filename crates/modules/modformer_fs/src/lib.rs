use anyhow::Result;
use async_trait::async_trait;
use modformer::Reader;
use serde_json::Value;

#[derive(Debug)]
pub struct Fs {}

impl Fs {
    pub fn new() -> Self {
        Fs {}
    }
}

#[async_trait]
impl Reader for Fs {
    async fn read(&self, _fs: &mut Value) -> Result<()> {
        Ok(())
    }
}
