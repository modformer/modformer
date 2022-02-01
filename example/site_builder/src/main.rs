use anyhow::Result;
use modformer_cli::{
    Metadata,
    Runner,
};
use modformer_core::phases::Read;
use modformer_fs::Fs;

// App

#[tokio::main]
async fn main() -> Result<()> {
    let read = core();
    let metadata = metadata();

    Runner::new(metadata, read).run().await
}

// Core

fn core<'a>() -> Read<'a> {
    Read::new().then(Fs::new()).finalize()
}

// Metadata

fn metadata<'a>() -> Metadata<'a> {
    Metadata::new("Some Author", "Some Description", "1.0.0")
}
