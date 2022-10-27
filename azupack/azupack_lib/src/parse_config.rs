use std::{fmt, fs, error::Error};
use serde::{Deserialize};
use serde_json;
use error_stack::{IntoReport, Result, ResultExt};

#[derive(Debug)]
pub struct ParseConfigError;

impl fmt::Display for ParseConfigError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Parsing package configuration: invalid configuration schema!")
    }
}

impl Error for ParseConfigError {}

/* --- Data structures ------------------------------------------------------ */

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Package {
    pub organization: String,
    pub feed: String,
    pub name: String,
    pub version: String,
    pub path: String
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct AzuPackConfig {
    pub packages: Vec<Package>
}


impl Package {
    pub fn new<S>(organization: S, feed: S, name: S, version: S, path: S) -> Package where S: Into<String> {
        Package {
            organization: organization.into(),
            feed: feed.into(),
            name: name.into(),
            version: version.into(),
            path: path.into()
        }
    }
}

impl AzuPackConfig {
    pub fn new() -> AzuPackConfig {
        AzuPackConfig { packages: Vec::new() }
    }
}

/* --- Functions ------------------------------------------------------------ */

pub fn parse_config(file_path: String) -> Result<AzuPackConfig, ParseConfigError> {
    /* read the file from the disk */
    let contents = fs::read_to_string(&file_path)
        .into_report()
        .attach_printable_lazy(|| {
            format!("Configuration file '{file_path:?}' could not read.")
        })
        .change_context(ParseConfigError)?;

    /* deserialize the configuration */
    let config: AzuPackConfig = serde_json::from_str(&contents)
        .into_report()
        .attach_printable_lazy(|| {
            format!("Configuration file '{file_path:?}' has an invalid schema.")
        })
        .change_context(ParseConfigError)?;

    return Ok(config);
}