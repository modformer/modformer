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
        .args([arg_verbosity()])
        .subcommands([subcommand_build(&name, metadata)])
}

// Args

fn arg_verbosity<'a>() -> Arg<'a> {
    Arg::new("verbosity")
        .long("verbosity")
        .short('v')
        .env("VERBOSITY")
        .takes_value(true)
        .value_name("LEVEL")
        .possible_values(&["debug", "info", "warn", "error", "none"])
        .default_value("none")
        .number_of_values(1)
        .help("Sets the level of logging output")
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
