use chrono::NaiveDate;
use semver::Version;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Change {
    Added(String),
    Changed(String),
    Deprecated(String),
    Removed(String),
    Fixed(String),
    Security(String),
}

#[derive(Debug, Fail)]
pub enum ChangeError {
    #[fail(display = "invalid change type specified: {}", _0)]
    InvalidChangeType(String),
}

#[derive(Debug, Clone, Builder)]
pub struct Release {
    #[builder(setter(strip_option), default)]
    version: Option<Version>,
    #[builder(setter(strip_option, into), default)]
    link: Option<String>,
    #[builder(setter(strip_option), default)]
    date: Option<NaiveDate>,
    #[builder(default)]
    changes: Vec<Change>,
    #[builder(default = "false")]
    yanked: bool,
}

#[derive(Debug, Clone, Builder)]
pub struct Changelog {
    #[builder(setter(into))]
    title: String,
    #[builder(setter(into))]
    description: String,
    #[builder(default)]
    releases: Vec<Release>,
}

impl Change {
    pub fn new(change_type: &str, description: String) -> Result<Self, ChangeError> {
        use self::Change::*;

        match change_type {
            "Added" => Ok(Added(description)),
            "Changed" => Ok(Changed(description)),
            "Deprecated" => Ok(Deprecated(description)),
            "Removed" => Ok(Removed(description)),
            "Fixed" => Ok(Fixed(description)),
            "Security" => Ok(Security(description)),
            _ => Err(ChangeError::InvalidChangeType(change_type.to_string())),
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
                fmt.write_str(&format!("{} - {} [YANKED]\n", version, date))?;
            } else {
                fmt.write_str(&format!("[{}] - {}\n", version, date))?;
            }
        } else {
            fmt.write_str("[Unreleased]\n")?;
        }

        // Release changes.
        let mut changesets: HashMap<&str, Vec<&Change>> = HashMap::new();
        self.changes.iter().for_each(|change| match change {
            Added(_) => match changesets.get_mut("Added") {
                Some(changes) => changes.push(change),
                None => {
                    changesets.insert("Added", vec![change]);
                }
            },
            Changed(_) => match changesets.get_mut("Changed") {
                Some(changes) => changes.push(change),
                None => {
                    changesets.insert("Changed", vec![change]);
                }
            },
            Deprecated(_) => match changesets.get_mut("Deprecated") {
                Some(changes) => changes.push(change),
                None => {
                    changesets.insert("Deprecated", vec![change]);
                }
            },
            Removed(_) => match changesets.get_mut("Removed") {
                Some(changes) => changes.push(change),
                None => {
                    changesets.insert("Removed", vec![change]);
                }
            },
            Fixed(_) => match changesets.get_mut("Fixed") {
                Some(changes) => changes.push(change),
                None => {
                    changesets.insert("Fixed", vec![change]);
                }
            },
            Security(_) => match changesets.get_mut("Security") {
                Some(changes) => changes.push(change),
                None => {
                    changesets.insert("Security", vec![change]);
                }
            },
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

        for (version, link) in links {
            fmt.write_str(&format!("[{}]: {}\n", version, link))?;
        }

        Ok(())
    }
}
