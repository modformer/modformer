use clap::{
    App,
    AppSettings,
    Arg,
};

use crate::{
    utils,
    Metadata,
};

// Content

const CONTENT_AFTER: &str = std::include_str!("./content/after.txt");
const CONTENT_BEFORE: &str = std::include_str!("./content/before.txt");

// App

pub(crate) fn new<'a>(metadata: &Metadata<'a>) -> App<'a> {
    let name = utils::get_exec_name().expect("exec name expected");

    App::new(&name)
        .about(metadata.description)
        .author(metadata.author)
        .version(metadata.version)
        .after_help(CONTENT_AFTER)
        .before_help(CONTENT_BEFORE)
        .bin_name(&name)
        .global_setting(AppSettings::PropagateVersion)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .args([arg_tracing(), arg_tracing_pretty()])
        .subcommands([subcommand_build(&name, metadata)])
}

// Args

fn arg_tracing<'a>() -> Arg<'a> {
    Arg::new("tracing")
        .long("tracing")
        .short('t')
        .env("TRACING")
        .takes_value(true)
        .value_name("LEVEL")
        .possible_values(&["trace", "debug", "info", "warn", "error"])
        .default_missing_value("info")
        .min_values(0)
        .max_values(1)
        .require_equals(true)
        .help("Set level for tracing output")
}

fn arg_tracing_pretty<'a>() -> Arg<'a> {
    Arg::new("tracing_pretty")
        .long("pretty")
        .short('p')
        .max_occurrences(1)
        .help("Enable pretty printing for tracing output")
}

// Subcommands

fn subcommand_build<'a>(name: &str, metadata: &Metadata<'a>) -> App<'a> {
    App::new("build")
        .about("Build the site using provided configuration")
        .author(metadata.author)
        .version(metadata.version)
        .after_help(CONTENT_AFTER)
        .before_help(CONTENT_BEFORE)
        .bin_name(name)
}
