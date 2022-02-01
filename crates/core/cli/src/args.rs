use clap::{
    App,
    AppSettings,
};

use crate::{
    utils,
    Metadata,
};

pub(crate) fn app<'a>(metadata: &Metadata<'a>) -> App<'a> {
    let name = utils::get_exec_name().expect("exec name expected");
    let info = "Built with Modformer (https://modformer.com)";

    App::new(&name)

        // Metadata and Display
        .about(metadata.description)
        .after_help(info)
        .author(metadata.author)
        .version(metadata.version)

        // Program Name
        .bin_name(&name)

        // Settings
        .global_setting(AppSettings::PropagateVersion)
        .setting(AppSettings::SubcommandRequiredElseHelp)

        // Subcommands
        .subcommands([
            App::new("build")
                .about("Build the site using provided configuration")
                .bin_name(&name),
        ])
}
