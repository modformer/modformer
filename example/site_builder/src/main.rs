#![forbid(unsafe_code)]

use anyhow::Result;
// use modformer::{
//     cli::{Runner, RunnerMetadata},
//     transformer::Transformer,
// };
// use modformer_fs::Fs;
use modformer::fs::*;

#[tokio::main]
async fn main() -> Result<()> {
    let fs = FileSystem::new();
    let _ = Directory::create_dir(&fs, "test_1").await?;
    let _ = Directory::create_file(&fs, "test_2", "Hello World".into()).await?;

    dbg!(fs);

    // let metadata = RunnerMetadata::new("Some Author", "Some Description",
    // "1.0.0"); let transformer = Transformer::build()
    //     .with_read(|rs| rs.push(Fs::new()).collect())
    //     .with_transform(|ts| ts.collect())
    //     .with_write(|ws| ws.collect())
    //     .finalize();

    // Runner::new(metadata, transformer).run().await

    Ok(())
}
