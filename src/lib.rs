extern crate pulldown_cmark;
extern crate chrono;
#[macro_use] extern crate derive_builder;

use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;
use semver::Version;
use chrono::NaiveDate;
use pulldown_cmark::{Parser, Event, Tag, LinkType};

#[derive(Debug, Clone)]
pub enum Change {
    Added(String),
    Changed(String),
    Deprecated(String),
    Removed(String),
    Fixed(String),
    Security(String),
}

impl Change {
    pub fn new(change_type: &str, description: String) -> Result<Self, ()> {
        match change_type {
            "Added" => Ok(Change::Added(description)),
            "Changed" => Ok(Change::Changed(description)),
            "Deprecated" => Ok(Change::Deprecated(description)),
            "Removed" => Ok(Change::Removed(description)),
            "Fixed" => Ok(Change::Fixed(description)),
            "Security" => Ok(Change::Security(description)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Builder)]
pub struct Release {
    #[builder(setter(strip_option), default)]
    version: Option<Version>,
    #[builder(setter(into))]
    link: String,
    #[builder(setter(strip_option), default)]
    date: Option<NaiveDate>,
    #[builder(default)]
    changes: Vec<Change>,
    #[builder(default = "false")]
    yanked: bool,
}

#[derive(Clone, Debug, PartialEq)]
enum ChangelogSection {
    None,
    Title,
    Description,
    ReleaseHeader,
    ChangesetHeader,
    Changeset(String),
}

#[derive(Debug)]
pub struct Changelog {
    title: String,
    description: String,
    releases: Vec<Release>,
}

impl Changelog {
    pub fn new(path: PathBuf) -> Self {
        let mut document = String::new();

        File::open(path).unwrap()
            .read_to_string(&mut document).unwrap();

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
                        },
                        ChangelogSection::Changeset(_) => {
                            release.changes(changeset.clone());
                            releases.push(release.build().unwrap());

                            changeset = Vec::new();
                            release = ReleaseBuilder::default();
                        },
                        _ => (),
                    }

                    section = ChangelogSection::ReleaseHeader;
                },
                Event::Start(Tag::Header(3)) => section = ChangelogSection::ChangesetHeader,

                // Links.
                Event::Start(Tag::Link(LinkType::Inline, _, _)) =>  accumulator.push_str("["),
                Event::End(Tag::Link(LinkType::Inline, href, _)) => accumulator.push_str(&format!("]({})", href)),
                Event::Start(Tag::Link(LinkType::Shortcut, href, _)) => {
                    release.link(href.to_string());
                },

                // Items.
                Event::End(Tag::Item) => {
                    if let ChangelogSection::Changeset(name) = section.clone() {
                        changeset.push(Change::new(&name, accumulator).unwrap());

                        accumulator = String::new();
                    }
                }

                // Line breaks.
                Event::SoftBreak => accumulator.push_str("\n"),
                Event::End(Tag::Paragraph) => accumulator.push_str("\n\n"),

                // Text.
                Event::Text(text) => {
                    match section {
                        ChangelogSection::Title => title = text.to_string(),
                        ChangelogSection::Description => accumulator.push_str(&text),
                        ChangelogSection::ReleaseHeader => {
                            let text = text.trim();

                            if text == "YANKED" {
                                release.yanked(true);
                            }

                            match NaiveDate::parse_from_str(&text, "- %Y-%m-%d") {
                                Ok(date) => {
                                    release.date(date);
                                },
                                _ => (),
                            }

                            match Version::parse(&text) {
                                Ok(version) => {
                                    release.version(version);
                                },
                                _ => (),
                            }
                        },
                        ChangelogSection::ChangesetHeader => section = ChangelogSection::Changeset(text.to_string()),
                        ChangelogSection::Changeset(_) => accumulator.push_str(&text),
                        _ => (),
                    }
                }
                _ => (),
            };
        }

        Changelog {
            title,
            description,
            releases
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cl = Changelog::new(PathBuf::from("test_changelog.md"));
        dbg!(cl);
    }
}
