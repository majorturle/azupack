use std::process::Command;
use std::{fmt, error::Error};
use error_stack::{IntoReport, Result, ResultExt};
use regex::Regex;

#[derive(Debug)]
pub struct AzCliComponent {
    pub name: String,
    pub version: String
}

#[derive(Debug)]
pub struct AzCli {
    components: Vec<AzCliComponent>
}

/* --- custom error types --------------------------------------------------- */
#[derive(Debug)]
pub struct AzCliError;

impl fmt::Display for AzCliError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Azure CLI: command execution error.")
    }
}

impl Error for AzCliError {}

impl AzCli {
    pub fn new() -> Result<AzCli, AzCliError> {
        let result = Command::new("az")
            .arg("--version")
            .output()
            .into_report()
            .attach_printable_lazy(|| {
                format!("Obtain the version of the Azure CLI.")
            })
            .change_context(AzCliError)?;

        let output = String::from_utf8(result.stdout)
            .into_report()
            .change_context(AzCliError)?;

        let mut cli = AzCli {
            components: Vec::new()
        };

        // extract information from the azure-cli
        // todo: not reading components correctly
        let re = Regex::new(r"([a-zA-Z-]+)\s+(\d+.\d+.\d+\w*)").unwrap();
        for cap in re.captures_iter(output.as_str()) {
            cli.components.push(
                AzCliComponent { name: String::from(&cap[1]), version: String::from(&cap[1]) }
            );
        }

        println!("{}", output);

        return Ok(cli);
    }
}