use anyhow::Result;
use modformer_core::phases::Read;

pub struct ModformerCli<'a> {
    _read: Read<'a>,
}

impl<'a> ModformerCli<'a> {
    pub fn new(read: Read<'a>) -> Self {
        Self { _read: read }
    }

    pub async fn build(&self) -> Result<()> {
        Ok(())
    }
}
