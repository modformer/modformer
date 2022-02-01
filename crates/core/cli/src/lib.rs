mod args;
mod utils;

use anyhow::Result;
use modformer_core::phases::Read;

// Metadata

pub struct Metadata<'a> {
    author: &'a str,
    description: &'a str,
    version: &'a str,
}

impl<'a> Metadata<'a> {
    pub fn new(
        author: impl Into<&'a str>,
        description: impl Into<&'a str>,
        version: impl Into<&'a str>,
    ) -> Self {
        Self {
            author: author.into(),
            description: description.into(),
            version: version.into(),
        }
    }
}

// Runner

pub struct Runner<'a> {
    metadata: Metadata<'a>,
    _read: Read<'a>,
}

impl<'a> Runner<'a> {
    pub fn new(metadata: Metadata<'a>, read: Read<'a>) -> Self {
        Self {
            metadata,
            _read: read,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let app = args::app(&self.metadata);
        let _matches = app.get_matches();

        Ok(())
    }
}
