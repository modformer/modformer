#![forbid(unsafe_code)]

use anyhow::Result;
use modformer::{
    cli::{
        Runner,
        RunnerMetadata,
    },
    transformer::Transformer,
};
use modformer_fs::Fs;

#[tokio::main]
async fn main() -> Result<()> {
    let metadata = RunnerMetadata::new("Some Author", "Some Description", "1.0.0");
    let transformer = Transformer::build()
        .with_read(|rs| rs.push(Fs::new()).collect())
        .with_transform(|ts| ts.collect())
        .with_write(|ws| ws.collect())
        .finalize();

    Runner::new(metadata, transformer).run().await
}
