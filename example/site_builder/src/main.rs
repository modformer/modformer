use anyhow::Result;
use modformer::{
    cli::{
        Runner,
        RunnerMetadata,
    },
    transformer::phases::Read,
};
use modformer_fs::Fs;

// App

#[tokio::main]
async fn main() -> Result<()> {
    let metadata = metadata();
    let transformer = transformer();

    Runner::new(metadata, transformer).run().await
}

// Metadata

fn metadata<'a>() -> RunnerMetadata<'a> {
    RunnerMetadata::new("Some Author", "Some Description", "1.0.0")
}

// Transformer

fn transformer<'a>() -> Read<'a> {
    Read::new().then(Fs::new()).finalize()
}
