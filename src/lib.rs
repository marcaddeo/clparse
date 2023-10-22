use anyhow::Result;
use changelog::{Change, Changelog, ChangelogBuilder, Release, ReleaseBuilder};
use chrono::NaiveDate;
use err_derive::Error;
use pulldown_cmark::{Event, LinkType, Parser, Tag};
use semver::Version;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub mod changelog;

#[derive(Clone, Debug)]
enum ChangelogFormat {
    Markdown,
    Json,
    Yaml,
}

#[derive(Clone, Debug)]
enum ChangelogSection {
    None,
    Title,
    Description,
    ReleaseHeader,
    ChangesetHeader,
    Changeset(String),
}

#[derive(Debug, Error)]
pub enum ChangelogParserError {
    #[error(display = "unable to determine file format from contents")]
    UnableToDetermineFormat,
    #[error(display = "error building release")]
    ErrorBuildingRelease(String),
}

pub struct ChangelogParser {
    separator: String,
}

impl ChangelogParser {
    pub fn new(separator: String) -> Self {
        Self {
            separator
        }
    }

    pub fn parse(&self, path: PathBuf) -> Result<Changelog> {
        let mut document = String::new();
        File::open(path.clone())?.read_to_string(&mut document)?;
        self.parse_buffer(document)
    }

    pub fn parse_buffer(&self, buffer: String) -> Result<Changelog> {
        match Self::get_format_from_buffer(buffer.clone()) {
            Ok(format) => match format {
                ChangelogFormat::Markdown => self.parse_markdown(buffer),
                ChangelogFormat::Json => Self::parse_json(buffer),
                ChangelogFormat::Yaml => Self::parse_yaml(buffer),
            },
            _ => Err(ChangelogParserError::UnableToDetermineFormat.into()),
        }
    }

    fn parse_markdown(&self, markdown: String) -> Result<Changelog> {
        let parser = Parser::new(&markdown);

        let mut section = ChangelogSection::None;

        let mut title = String::new();
        let mut description = String::new();
        let mut description_links = String::new();
        let mut releases: Vec<Release> = Vec::new();

        let mut release = ReleaseBuilder::default();
        let mut changeset: Vec<Change> = Vec::new();
        let mut accumulator = String::new();
        let mut link_accumulator = String::new();

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
                        ChangelogSection::Changeset(_) | ChangelogSection::ReleaseHeader => {
                            release.changes(changeset.clone());
                            release.separator(self.separator.clone());
                            releases.push(
                                release
                                    .build()
                                    .map_err(ChangelogParserError::ErrorBuildingRelease)?,
                            );

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
                Event::Start(Tag::Link(LinkType::Collapsed, _, _)) => {
                    accumulator.push_str("[");
                    link_accumulator = String::from("[");
                }
                Event::End(Tag::Link(LinkType::Inline, href, _)) => {
                    accumulator.push_str(&format!("]({})", href));
                }
                Event::End(Tag::Link(LinkType::Collapsed, href, _)) => {
                    accumulator.push_str("][]");
                    link_accumulator.push_str(&format!("]: {}\n", href));
                    description_links.push_str(&link_accumulator);
                    link_accumulator = String::new();
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
                Event::Code(text) => accumulator.push_str(&format!("`{}`", text)),

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
                    ChangelogSection::Description => {
                        accumulator.push_str(&text);

                        if !link_accumulator.is_empty() {
                            link_accumulator.push_str(&text);
                        }
                    }
                    ChangelogSection::ReleaseHeader => {
                        let text = text.trim();

                        if text == "YANKED" {
                            release.yanked(true);
                        }

                        let mut date_format = format!("{} %Y-%m-%d", self.separator);
                        let split: Vec<&str> = text.split(&format!(" {} ", self.separator)).collect();

                        if split.iter().count() > 1 {
                            date_format = "%Y-%m-%d".into();
                        }

                        for string in split {
                            if let Ok(date) = NaiveDate::parse_from_str(&string, &date_format) {
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
        release.separator(self.separator.clone());
        releases.push(
            release
                .build()
                .map_err(ChangelogParserError::ErrorBuildingRelease)?,
        );

        if !description_links.is_empty() {
            description = format!("{}{}\n", description, description_links);
        }

        let changelog = ChangelogBuilder::default()
            .title(title)
            .description(description)
            .releases(releases)
            .build()
            .map_err(ChangelogParserError::ErrorBuildingRelease)?;

        Ok(changelog)
    }

    fn parse_json(json: String) -> Result<Changelog> {
        let changelog: Changelog = serde_json::from_str(&json)?;

        Ok(changelog)
    }

    fn parse_yaml(yaml: String) -> Result<Changelog> {
        let changelog: Changelog = serde_yaml::from_str(&yaml)?;

        Ok(changelog)
    }

    fn get_format_from_buffer(buffer: String) -> Result<ChangelogFormat> {
        let first_char = match buffer.chars().next() {
            Some(first_char) => first_char,
            _ => {
                return Err(ChangelogParserError::UnableToDetermineFormat.into());
            }
        };

        let first_line: String = buffer.chars().take_while(|&c| c != '\n').collect();
        let mut format: Option<ChangelogFormat> = match first_char {
            '{' => Some(ChangelogFormat::Json),
            '#' => Some(ChangelogFormat::Markdown),
            _ => None,
        };

        if format.is_none() && (first_line == "---" || first_line.contains("title:")) {
            format = Some(ChangelogFormat::Yaml);
        }

        if let Some(format) = format {
            Ok(format)
        } else {
            Err(ChangelogParserError::UnableToDetermineFormat.into())
        }
    }
}
