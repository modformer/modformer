mod app;
mod utils;

use anyhow::Result;
use clap::ArgMatches;
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
}

impl<'a> Runner<'a> {
    pub async fn run(&self) -> Result<()> {
        match app::new(&self.metadata).get_matches().subcommand() {
            Some(("build", arg_matches)) => self.build(arg_matches).await,
            _ => Ok(()),
        }
    }

    async fn build(&self, arg_matches: &ArgMatches) -> Result<()> {
        println!("Build: {:#?}", arg_matches);

        Ok(())
    }
}
