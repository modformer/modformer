use std::fmt::Debug;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

// Module

#[async_trait]
pub trait Read: Debug {
    async fn read(&self, fs: &mut Value) -> Result<()>;
}

#[async_trait]
pub trait Transform: Debug {
    async fn transform(&self, fs: &mut Value) -> Result<()>;
}

#[async_trait]
pub trait Write: Debug {
    async fn write(&self, fs: &mut Value) -> Result<()>;
}
