use anyhow::Result;
use modformer_cli::{
    Metadata,
    Runner,
};
use modformer_fs::Fs;
use modformer_transformer::phases::Read;

// App

#[tokio::main]
async fn main() -> Result<()> {
    let metadata = metadata();
    let transformer = transformer();

    Runner::new(metadata, transformer).run().await
}

// Metadata

fn metadata<'a>() -> Metadata<'a> {
    Metadata::new("Some Author", "Some Description", "1.0.0")
}

// Transformer

fn transformer<'a>() -> Read<'a> {
    Read::new().then(Fs::new()).finalize()
}
