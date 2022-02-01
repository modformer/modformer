use anyhow::Result;
use modformer_cli::ModformerCli;
use modformer_core::phases::Read;
use modformer_fs::Fs;

#[tokio::main]
async fn main() -> Result<()> {
    ModformerCli::new(Read::new().then(Fs::new()).finalize())
        .build()
        .await
}
