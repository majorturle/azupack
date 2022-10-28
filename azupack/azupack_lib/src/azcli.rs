use std::io::Write;
use std::process::{Command, Stdio};
use std::{fmt, error::Error};
use error_stack::{IntoReport, Result, ResultExt, report};
use regex::Regex;
use crate::login::{LoginToken};
use crate::parse_config::Package;

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

#[derive(Debug)]
pub struct PackageDownloadError;

impl fmt::Display for PackageDownloadError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Azure CLI: error downloading a package.")
    }
}

impl Error for PackageDownloadError {}

#[derive(Debug)]
pub struct AzLoginError;

impl fmt::Display for AzLoginError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Azure CLI: cannot login to DevOps cli.")
    }
}

impl Error for AzLoginError {}

/* --- functions ------------------------------------------------------------ */

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

    pub fn login(&mut self, token: &LoginToken) -> Result<(), AzLoginError> {
        // note: az is the executable and takes all the arguments as a single argument!
        // the sub-arguments are forwarded to the subprograms
        let mut child = Command::new("az")
            .arg("devops").arg("login").arg("--org").arg(token.organization.as_str())
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .spawn()
            .into_report()
            .attach_printable_lazy(|| {
                format!("Cannot spawn process 'az devops login'.")
            }).change_context(AzLoginError)?;

            let child_stdin = child.stdin.as_mut().unwrap();
            child_stdin.write_all(token.token.as_bytes())
                .into_report()
                .change_context(AzLoginError)?;

            // Close stdin to finish and avoid indefinite blocking
            drop(child_stdin);
            let output = child.wait_with_output()
                .into_report().change_context(AzLoginError)?;

            if let Some(exit_code) = output.status.code() {
                if exit_code != 0 {
                    return Err(report!(AzLoginError))
                    .attach_printable_lazy(|| {
                        format!("Invalid PAT for organization '{}'", token.organization)
                    })
                }
            } else {
                return Err(report!(AzLoginError))
                    .attach_printable_lazy(|| {
                        format!("Login process was killed for organization '{}'", token.organization)
                    })
            }

            println!("Login successful!");
            Ok(())
    }

    pub fn download(&self, package: &Package) -> Result<(), PackageDownloadError> {
        let mut child = Command::new("az")
        .args(vec!["artifacts", "universal", "download",
                   "--organization", package.organization.as_str(),
                   "--feed", package.feed.as_str(),
                   "--name", package.name.as_str(),
                   "--version", package.version.as_str(),
                   "--path", package.path.as_str() ])
        .output()
        .into_report()
        .attach_printable_lazy(|| {
            format!("Error while downloading the package {}.", package.name)
        }).change_context(PackageDownloadError)?;

        // TODO: add the console log if the this was not successful
        

        Ok(())
    }
}