use yaml_rust::{Yaml, YamlLoader};
use std::fs;
use crate::{SITES_KEY, TIMING_KEY};
use std::collections::HashSet;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Config {
    pub sites: HashSet<String>,
    pub period: Duration,
}

impl Config {
    fn new(sites: HashSet<String>, period: Duration) -> Self {
        Config {
            sites,
            period,
        }
    }
}

pub fn parse_config_file(path: &str) -> Result<Config, String> {
    let yaml: String = match fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(_) => return Result::Err(format!("Error: '{}' doesn't point to a file we can open.", path)),
    };
    let yaml = match YamlLoader::load_from_str(&yaml) {
        Ok(yaml) => yaml,
        Err(_) => return Result::Err(format!("Error: Unable to parse that YAML file. Here are the contents: {}", &yaml)),
    };
    let hash = match yaml.get(0) {
        None => return Result::Err(format!("Error: .yaml config file has no contents")),

        Some(hash) => hash
    };
    let hash = match hash.as_hash() {
        None => return Result::Err(format!("Error: .yaml config file must be a hash at the top level.")),
        Some(hash) => hash
    };

    let sites = match &hash[&Yaml::String(String::from("sites"))] {
        Yaml::Array(arr) => arr,
        _ => return Result::Err(format!("Error: {} key in .yaml config file must contain an array of URLs (strings)", SITES_KEY))
    };

    let mut cleaned_sites: HashSet<String> = Default::default();

    for site in sites.iter() {
        match site {
            Yaml::String(value) => {
                cleaned_sites.insert(value.clone());
            }
            _ => return Result::Err(format!("Error: Site: {:?} is not a string", site))
        }
    }

    let freq: u64 = match &hash[&Yaml::String(String::from("period_ms"))] {
        Yaml::Real(num) => match num.parse::<u64>() {
            Ok(num) => num,
            Err(_) => return Result::Err(format!("Error: Unable to parse '{}' into an integer in period_ms in your config file.", num))

        },
        Yaml::Integer(num) => *num as u64,
        _ => return Result::Err(format!("Error: {} must contain a number like 5000 or 10000 (5 seconds and 10 seconds)", TIMING_KEY))
    };

    return Result::Ok(Config::new(cleaned_sites, Duration::from_millis(freq)));
}