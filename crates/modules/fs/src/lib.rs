use anyhow::Result;
use async_trait::async_trait;
use modformer_traits::Reader;
use serde_json::Value;

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
