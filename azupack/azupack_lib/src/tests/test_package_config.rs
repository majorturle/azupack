use crate::package_config::{parse_config, AzuPackConfig, Package};

#[test]
fn config_parse_structure() {
    // parse a test config file
    let config = parse_config(String::from("./data/test_azupack.json"))
        .unwrap();

    // check if the parsed data is correct
    let mut config_gold = AzuPackConfig::new();
    config_gold.packages.push(
        Package::new("https://dev.azure.com/LlamaCo", "LlamaFeed", "ZooPackage", "1.*", "./download")
    );
    config_gold.packages.push(
        Package::new("https://dev.azure.com/TurtleCo", "TurleFeed", "WaterPackage", "1.9.22", "./download")
    );

    assert_eq!(config, config_gold);

}