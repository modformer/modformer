mod app;
mod utils;

use anyhow::Result;
use clap::ArgMatches;
use modformer_transformer::Transformer;
use time::format_description;
use tracing::{
    debug,
    info,
    instrument,
    subscriber,
};
use tracing_subscriber::{
    fmt::time::UtcTime,
    EnvFilter,
};

// Metadata

#[derive(Debug)]
pub struct RunnerMetadata<'a> {
    author: &'a str,
    description: &'a str,
    version: &'a str,
}

impl<'a> RunnerMetadata<'a> {
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

#[derive(Debug)]
pub struct Runner<'a> {
    metadata: RunnerMetadata<'a>,
    _transformer: Transformer,
}

impl<'a> Runner<'a> {
    pub fn new(metadata: RunnerMetadata<'a>, transformer: Transformer) -> Self {
        Self {
            metadata,
            _transformer: transformer,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let app = app::new(&self.metadata);
        let matches = app.get_matches();

        self.init(&matches).await?;
        self.exec(&matches).await
    }
}

const TIME_FORMAT: &str = "[month repr:short] [day] [hour]:[minute]:[second].[subsecond digits:3]";

impl<'a> Runner<'a> {
    async fn init(&self, matches: &ArgMatches) -> Result<()> {
        self.init_tracing(matches).await?;

        Ok(())
    }

    async fn init_tracing(&self, matches: &ArgMatches) -> Result<()> {
        if let Some(level) = matches.value_of("tracing") {
            let filter = EnvFilter::try_new(level)?;
            let format = format_description::parse(TIME_FORMAT)?;
            let timer = UtcTime::new(format);

            let subscriber = tracing_subscriber::fmt()
                .with_env_filter(filter)
                .with_line_number(true)
                .with_thread_names(true)
                .with_timer(timer);

            match matches.occurrences_of("tracing_pretty") {
                0 => subscriber::set_global_default(subscriber.finish())?,
                _ => subscriber::set_global_default(subscriber.pretty().finish())?,
            }
        }

        Ok(())
    }
}

impl<'a> Runner<'a> {
    #[instrument(err, level = "trace")]
    async fn exec(&self, matches: &ArgMatches) -> Result<()> {
        debug!("Starting subcommand match...");

        match matches.subcommand() {
            Some(("build", matches)) => self.exec_build(&matches).await,
            _ => Ok(()),
        }
    }

    #[instrument(err, level = "trace")]
    async fn exec_build(&self, matches: &ArgMatches) -> Result<()> {
        info!("Starting build...");

        Ok(())
    }
}
