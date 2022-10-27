use crate::azcli::{AzCli, AzCliError};

#[test]
fn azcli_version() {
    AzCli::new().unwrap();
}