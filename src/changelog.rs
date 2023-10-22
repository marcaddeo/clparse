use anyhow::Result;
use chrono::NaiveDate;
use derive_builder::Builder;
use derive_getters::Getters;
use err_derive::Error;
use indexmap::indexmap;
use versions::Version;
use serde::ser::Serializer;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use textwrap::wrap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Change {
    Added(String),
    Changed(String),
    Deprecated(String),
    Removed(String),
    Fixed(String),
    Security(String),
}

#[derive(Debug, Error)]
pub enum ChangeError {
    #[error(display = "invalid change type specified: {}", _0)]
    InvalidChangeType(String),
}

fn version_serialize<S>(x: &Option<Version>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match x {
        Some(ref version) => s.serialize_str(&version.to_string()),
        None => s.serialize_none(),
    }
}

#[derive(Debug, Clone, Builder, Getters, Serialize, Deserialize, PartialEq)]
pub struct Release {
    #[builder(setter(strip_option), default)]
    #[serde(serialize_with = "version_serialize")]
    version: Option<Version>,
    #[builder(setter(strip_option, into), default)]
    link: Option<String>,
    #[builder(setter(strip_option), default)]
    date: Option<NaiveDate>,
    #[builder(default)]
    changes: Vec<Change>,
    #[builder(default = "false")]
    yanked: bool,
    #[serde(skip)]
    #[builder(default = "self.default_separator()")]
    separator: String,
}

impl ReleaseBuilder {
    fn default_separator(&self) -> String {
        "-".into()
    }
}

impl Release {
    pub fn version_mut(&mut self) -> &mut Option<Version> {
        &mut self.version
    }

    pub fn set_version(&mut self, version: Version) -> &mut Self {
        self.version = Some(version);
        self
    }

    pub fn link_mut(&mut self) -> &mut Option<String> {
        &mut self.link
    }

    pub fn set_link(&mut self, link: String) -> &mut Self {
        self.link = Some(link);
        self
    }

    pub fn date_mut(&mut self) -> &mut Option<NaiveDate> {
        &mut self.date
    }

    pub fn set_date(&mut self, date: NaiveDate) -> &mut Self {
        self.date = Some(date);
        self
    }

    pub fn changes_mut(&mut self) -> &mut Vec<Change> {
        &mut self.changes
    }

    pub fn set_changes(&mut self, changes: Vec<Change>) -> &mut Self {
        self.changes = changes;
        self
    }

    pub fn yank(&mut self, yanked: bool) {
        if !self.yanked && yanked {
            self.link = None;
        }

        self.yanked = yanked;
    }
}

#[derive(Debug, Clone, Builder, Getters, Serialize, Deserialize)]
pub struct Changelog {
    #[builder(setter(into))]
    title: String,
    #[builder(setter(into))]
    description: String,
    #[builder(default)]
    releases: Vec<Release>,
}

impl Changelog {
    pub fn unreleased_changes(&self) -> Vec<Change> {
        self.releases
            .clone()
            .into_iter()
            .filter(|r| r.version.is_none())
            .map(|r| r.changes.clone())
            .flatten()
            .collect()
    }

    pub fn unreleased_mut(&mut self) -> Option<&mut Release> {
        self.releases.iter_mut().find(|r| r.version == None)
    }

    pub fn release_mut(&mut self, release: Version) -> Option<&mut Release> {
        self.releases
            .iter_mut()
            .find(|r| r.version == Some(release.clone()))
    }
}

impl Change {
    pub fn new(change_type: &str, description: String) -> Result<Self> {
        use self::Change::*;

        match change_type.to_lowercase().as_str() {
            "added" => Ok(Added(description)),
            "changed" => Ok(Changed(description)),
            "deprecated" => Ok(Deprecated(description)),
            "removed" => Ok(Removed(description)),
            "fixed" => Ok(Fixed(description)),
            "security" => Ok(Security(description)),
            _ => Err(ChangeError::InvalidChangeType(change_type.to_string()).into()),
        }
    }
}

impl fmt::Display for Change {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Change::*;

        let description = match self {
            Added(description) => description,
            Changed(description) => description,
            Deprecated(description) => description,
            Removed(description) => description,
            Fixed(description) => description,
            Security(description) => description,
        };
        let description = description.as_str().replace("\n", " ");
        let description = wrap(&description, 77).join("\n  ");

        fmt.write_str(&format!("- {}\n", description))?;

        Ok(())
    }
}

impl fmt::Display for Release {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Change::*;

        // Release Heading.
        fmt.write_str("## ")?;

        // Release Version.
        if let (Some(version), Some(date)) = (self.version.as_ref(), self.date) {
            if self.yanked {
                fmt.write_str(&format!("{} {} {} [YANKED]\n", version, self.separator, date))?;
            } else {
                fmt.write_str(&format!("[{}] {} {}\n", version, self.separator, date))?;
            }
        } else {
            fmt.write_str("[Unreleased]\n")?;

            if self.changes.is_empty() {
                fmt.write_str("\n")?;
            }
        }

        // Release changes.
        let mut changesets = indexmap! {
            "Added" => Vec::new(),
            "Changed" => Vec::new(),
            "Deprecated" => Vec::new(),
            "Removed" => Vec::new(),
            "Fixed" => Vec::new(),
            "Security" => Vec::new(),
        };
        self.changes.iter().for_each(|change| match change {
            Added(_) => changesets.get_mut("Added").unwrap().push(change),
            Changed(_) => changesets.get_mut("Changed").unwrap().push(change),
            Deprecated(_) => changesets.get_mut("Deprecated").unwrap().push(change),
            Removed(_) => changesets.get_mut("Removed").unwrap().push(change),
            Fixed(_) => changesets.get_mut("Fixed").unwrap().push(change),
            Security(_) => changesets.get_mut("Security").unwrap().push(change),
        });

        changesets = changesets
            .into_iter()
            .filter(|(_, changes)| changes.clone().iter().count() > 0)
            .collect();

        for (name, changes) in changesets {
            fmt.write_str(&format!("### {}\n", name))?;

            for change in changes {
                fmt.write_str(&change.to_string())?;
            }

            fmt.write_str("\n")?;
        }

        Ok(())
    }
}

impl fmt::Display for Changelog {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&format!("# {}\n", self.title))?;
        fmt.write_str(&self.description)?;

        let mut links: Vec<(Version, String)> = Vec::new();
        for release in self.releases.clone() {
            fmt.write_str(&release.to_string())?;

            if let (Some(version), Some(link)) = (release.version, release.link) {
                links.push((version, link));
            }
        }

        links.sort_by(|(a, _), (b, _)| b.cmp(a));

        if let Some(release) = self.releases.clone().first() {
            if let (None, Some(link)) = (release.version.as_ref(), release.link.as_ref()) {
                fmt.write_str(&format!("[Unreleased]: {}\n", link))?
            }
        }

        for (version, link) in links {
            fmt.write_str(&format!("[{}]: {}\n", version, link))?;
        }

        Ok(())
    }
}
