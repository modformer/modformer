use anyhow::Result;
use modformer::{
    cli::{
        Runner,
        RunnerMetadata,
    },
    transformer::Modformer,
};
use modformer_fs::Fs;

#[tokio::main]
async fn main() -> Result<()> {
    let metadata = RunnerMetadata::new("Some Author", "Some Description", "1.0.0");
    let modformer = Modformer::build()
        .with_readers(|rs| rs.push(Fs::new()).collect())
        .with_transformers(|ts| ts.collect())
        .with_writers(|ws| ws.collect())
        .finalize();

    Runner::new(metadata, modformer).run().await
}
