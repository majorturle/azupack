use crate::{azcli::{AzCli}, login::LoginToken};

#[test]
fn azcli_version() {
    AzCli::new().unwrap();
}

#[test]
fn login() {
    let mut cli = AzCli::new().unwrap();

    let llama_token = LoginToken::new("https://dev.azure.com/LlamaCo","llamatoken");
    cli.login(&llama_token).unwrap();
}