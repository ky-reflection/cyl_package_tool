mod cylheim_tools;
use cylheim_tools::CylheimProjectConfig;
use std::fs;
use std::io::{BufRead, BufReader, Error, Write};
#[allow(dead_code)]

fn main() {
    let path = "./resource/CFHSS X.cyl";
    let text = fs::read_to_string(path).unwrap();
    let config: CylheimProjectConfig = serde_json::from_str(&text).unwrap();
    println!("{:#?}", config);
}
