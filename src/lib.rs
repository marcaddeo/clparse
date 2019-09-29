use changelog::{Change, Changelog, ChangelogBuilder, Release, ReleaseBuilder};
use chrono::NaiveDate;
use failure::Error;
use fstrings::*;
use pulldown_cmark::{Event, LinkType, Parser, Tag};
use semver::Version;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub mod changelog;

#[derive(Clone, Debug, PartialEq)]
enum ChangelogSection {
    None,
    Title,
    Description,
    ReleaseHeader,
    ChangesetHeader,
    Changeset(String),
}

pub struct ChangelogParser;
impl ChangelogParser {
    pub fn parse(path: PathBuf) -> Result<Changelog, Error> {
        let mut document = String::new();

        File::open(path)?.read_to_string(&mut document)?;

        let parser = Parser::new(&document);

        let mut section = ChangelogSection::None;

        let mut title = String::new();
        let mut description = String::new();
        let mut releases: Vec<Release> = Vec::new();

        let mut release = ReleaseBuilder::default();
        let mut changeset: Vec<Change> = Vec::new();
        let mut accumulator = String::new();

        for event in parser {
            match event {
                // Headings.
                Event::Start(Tag::Header(1)) => section = ChangelogSection::Title,
                Event::End(Tag::Header(1)) => section = ChangelogSection::Description,
                Event::Start(Tag::Header(2)) => {
                    match section {
                        ChangelogSection::Description => {
                            description = accumulator.clone();
                            accumulator = String::new();
                        }
                        ChangelogSection::Changeset(_) => {
                            release.changes(changeset.clone());
                            releases.push(release.build().unwrap());

                            changeset = Vec::new();
                            release = ReleaseBuilder::default();
                        }
                        _ => (),
                    }

                    section = ChangelogSection::ReleaseHeader;
                }
                Event::Start(Tag::Header(3)) => section = ChangelogSection::ChangesetHeader,

                // Links.
                Event::Start(Tag::Link(LinkType::Inline, _, _)) => accumulator.push_str("["),
                Event::End(Tag::Link(LinkType::Inline, href, _)) => {
                    accumulator.push_str(&f!("]({href})"));
                }
                Event::Start(Tag::Link(LinkType::Shortcut, href, _)) => {
                    release.link(href.to_string());
                }

                // Items.
                Event::End(Tag::Item) => {
                    if let ChangelogSection::Changeset(name) = section.clone() {
                        changeset.push(Change::new(&name, accumulator)?);

                        accumulator = String::new();
                    }
                }

                // Line breaks.
                Event::SoftBreak => accumulator.push_str("\n"),
                Event::End(Tag::Paragraph) => accumulator.push_str("\n\n"),

                // Inline code.
                Event::Code(text) => accumulator.push_str(&f!("`{text}`")),

                // Text formatting.
                Event::Start(Tag::Strong) | Event::End(Tag::Strong) => accumulator.push_str("**"),
                Event::Start(Tag::Emphasis) | Event::End(Tag::Emphasis) => {
                    accumulator.push_str("_")
                }
                Event::Start(Tag::Strikethrough) | Event::End(Tag::Strikethrough) => {
                    accumulator.push_str("~~")
                }

                // Text.
                Event::Text(text) => match section {
                    ChangelogSection::Title => title = text.to_string(),
                    ChangelogSection::Description => accumulator.push_str(&text),
                    ChangelogSection::ReleaseHeader => {
                        let text = text.trim();

                        if text == "YANKED" {
                            release.yanked(true);
                        }

                        let mut date_format = "- %Y-%m-%d";
                        let split: Vec<&str> = text.split(" - ").collect();

                        if split.iter().count() > 1 {
                            date_format = "%Y-%m-%d";
                        }

                        for string in split {
                            if let Ok(date) = NaiveDate::parse_from_str(&string, date_format) {
                                release.date(date);
                            }

                            if let Ok(version) = Version::parse(&string) {
                                release.version(version);
                            }
                        }
                    }
                    ChangelogSection::ChangesetHeader => {
                        section = ChangelogSection::Changeset(text.to_string())
                    }
                    ChangelogSection::Changeset(_) => accumulator.push_str(&text),
                    _ => (),
                },
                _ => (),
            };
        }

        release.changes(changeset.clone());
        releases.push(release.build().unwrap());

        let changelog = ChangelogBuilder::default()
            .title(title)
            .description(description)
            .releases(releases)
            .build()
            .unwrap();

        Ok(changelog)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate serde_json;

    #[test]
    fn it_works() {
        let cl = ChangelogParser::parse(PathBuf::from("test_changelog.md")).unwrap();

        println!("{}", serde_json::to_string_pretty(&cl).unwrap());
    }
}
