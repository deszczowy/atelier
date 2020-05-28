use serde_json::{Result, Value};
use std::fs::File;
use std::path::Path;

pub struct Config {}

pub trait Configurable {
    fn new(file_name: String) -> Result<Value> ;
}

impl Configurable for Config {
    fn new(file_name: String) -> Result<Value> {

        //let config_file_name = format!("{}{}", var("HOME").unwrap(), file_name);
        let config_file_path = Path::new(&file_name);

        let json = File::open(config_file_path).expect("config file not found");

        let cfg : Value = serde_json::from_reader(json).expect("error while reading json");
        Ok(cfg)
    }
}