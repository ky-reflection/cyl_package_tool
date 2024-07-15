mod cylheim_tools;
use cylheim_tools::{CylheimChart, CylheimProjectConfig};
use std::fs;
#[allow(dead_code)]

fn main() {
    let path = "./resource/test.json";
    let text = fs::read_to_string(path).unwrap();
    let config: CylheimChart = serde_json::from_str(&text).unwrap();
    // println!("{:#?}", config);
}
