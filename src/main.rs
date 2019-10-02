use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};
use clparse::ChangelogParser;
use failure::Error;
use std::io::{self, Read};

pub fn main() -> Result<(), Error> {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("format")
                .help("Sets the output format of the parsed CHANGELOG")
                .takes_value(true)
                .default_value("markdown")
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

    match matches.value_of("format").unwrap() {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&changelog)?);
        }
        "yaml" | "yml" => {
            println!("{}", serde_yaml::to_string(&changelog)?);
        }
        "markdown" | "md" => {
            println!("{}", &changelog);
        }
        _ => unreachable!(),
    }

    Ok(())
}
