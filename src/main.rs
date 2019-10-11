use anyhow::Result;
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, AppSettings, Arg,
};
use clparse::ChangelogParser;
use std::io::{self, Read, Write};

pub fn main() -> Result<()> {
    let matches = app_from_crate!()
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::ArgRequiredElseHelp)
        .global_setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("format")
                .help("Sets the output format of the parsed CHANGELOG [default: markdown]")
                .takes_value(true)
                .possible_values(&["json", "yaml", "yml", "markdown", "md"])
                .short("f")
                .long("format"),
        )
        .arg(
            Arg::with_name("file")
                .help("The CHANGELOG file to parse. This should be either a Markdown, JSON, or Yaml representation of a changelog. Use '-' to read from stdin.")
                .value_name("FILE")
                .index(1)
                .required(true),
        )
        .get_matches();

    let file = matches.value_of("file").unwrap();
    let changelog = if file == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;

        ChangelogParser::parse_buffer(buffer)?
    } else {
        ChangelogParser::parse(file.into())?
    };

    let output = match matches.value_of("format").unwrap_or("markdown") {
        "json" => {
            format!("{}", serde_json::to_string_pretty(&changelog)?)
        }
        "yaml" | "yml" => {
            format!("{}", serde_yaml::to_string(&changelog)?)
        }
        "markdown" | "md" => {
            format!("{}", &changelog)
        }
        _ => unreachable!(),
    };

    io::stdout().write_all(output.as_bytes())?;

    Ok(())
}
