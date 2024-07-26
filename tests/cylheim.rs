use cyl_package_tool::cylheim_tools::{CylheimChart, CylheimProjectConfig};
use serde_json;
use std::fs;
#[test]
fn test_cylheim_config_deserialize() {
    let path = "./tests/resources/test_cyl_config.cyl";
    let f = fs::read_to_string(path).unwrap();
    let config: CylheimProjectConfig = serde_json::from_str(&f).unwrap();
    println!("{:?}", config);
}
#[test]
fn test_cylheim_chart() {
    let path = "./tests/resources/test_cyl_chart.json";
    let f = fs::read_to_string(path).unwrap();
    let config: CylheimChart = serde_json::from_str(&f).unwrap();
    println!("{:?}", &config);
    let config_json = serde_json::to_string(&config).unwrap();
    println!("{:?}", &config_json);
}
