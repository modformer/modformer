use std::fmt::Debug;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

// Module

#[async_trait]
pub trait Reader: Debug {
    async fn read(&self, fs: &mut Value) -> Result<()>;
}

#[async_trait]
pub trait Transformer: Debug {
    async fn transform(&self, fs: &mut Value) -> Result<()>;
}

#[async_trait]
pub trait Writer: Debug {
    async fn write(&self, fs: &mut Value) -> Result<()>;
}
