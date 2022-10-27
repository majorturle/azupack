use crate::login::{LoginProvider, LoginToken};

#[test]
fn test_login_error() {
    let mut provider = LoginProvider::new();

    let llama_token = LoginToken::new("https://dev.azure.com/LlamaCo","llamatoken");
    let turtle_token = LoginToken::new("https://dev.azure.com/TurtleCo","turtletoken");
    provider.add_token(llama_token.clone());
    provider.add_token(turtle_token.clone());

    let is_err = provider.get_tokens("https://dev.azure.com/nonexistant")
        .is_err();

    assert_eq!(is_err, true);
}


#[test]
fn test_login_single() {
    let mut provider = LoginProvider::new();

    let llama_token = LoginToken::new("https://dev.azure.com/LlamaCo","llamatoken");
    let turtle_token = LoginToken::new("https://dev.azure.com/TurtleCo/","turtletoken");
    provider.add_token(llama_token.clone());
    provider.add_token(turtle_token.clone());

    let result = provider.get_tokens("https://dev.azure.com/LlamaCo").unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], &llama_token);

    // check for tolerance with the '/'
    let result = provider.get_tokens("https://dev.azure.com/LlamaCo/").unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], &llama_token);

    let result = provider.get_tokens("https://dev.azure.com/TurtleCo").unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], &turtle_token);
}

#[test]
fn test_login_multiple() {
    let mut provider = LoginProvider::new();

    let llama_token = LoginToken::new("https://dev.azure.com/LlamaCo","llamatoken");
    let llama_token2 = LoginToken::new("https://dev.azure.com/LlamaCo","llamatoken2");
    let turtle_token = LoginToken::new("https://dev.azure.com/TurtleCo","turtletoken");
    provider.add_token(llama_token.clone());
    provider.add_token(llama_token2.clone());
    provider.add_token(turtle_token.clone());

    let result = provider.get_tokens("https://dev.azure.com/LlamaCo").unwrap();

    // only find one instance with correct values
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], &llama_token);
    assert_eq!(result[1], &llama_token2);
}