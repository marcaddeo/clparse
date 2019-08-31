use chrono::NaiveDate;
use semver::Version;

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

#[derive(Debug, Builder)]
pub struct Changelog {
    #[builder(setter(into))]
    title: String,
    #[builder(setter(into))]
    description: String,
    #[builder(default)]
    releases: Vec<Release>,
}
