use anyhow::Result;
use async_trait::async_trait;
use modformer::model::Read;
use serde_json::Value;

#[derive(Debug)]
pub struct Fs {}

impl Fs {
    pub fn new() -> Self {
        Fs {}
    }
}

#[async_trait]
impl Read for Fs {
    async fn read(&self, _fs: &mut Value) -> Result<()> {
        Ok(())
    }
}
