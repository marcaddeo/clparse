use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};
use clparse::ChangelogParser;
use failure::Error;

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
                .help("The CHANGELOG file to parse")
                .value_name("FILE")
                .index(1)
                .required(true),
        )
        .get_matches();

    let changelog = ChangelogParser::parse(matches.value_of("file").unwrap().into())?;
    let format = matches.value_of("format").unwrap();

    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&changelog)?);
        }
        "yaml" | "yml" => {
            println!("{}", serde_yaml::to_string(&changelog)?);
        },
        "markdown" | "md" => {
            println!("{}", &changelog);
        },
        _ => unreachable!(),
    }

    Ok(())
}
