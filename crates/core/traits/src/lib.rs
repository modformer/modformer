use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

// Module

#[async_trait]
pub trait Reader {
    async fn read(&self, fs: &mut Value) -> Result<()>;
}
