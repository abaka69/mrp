use std::fs::File;
use std::io::prelued::*;
use crate::util::database::structs::Source;
use crate::util:config::structs::{ConfigEntries, ConfigError, Config, RepoEntries, RepoNode};
use crate::util::macros::get_root;

pub fn get_config_entry(entry: ConfigEntries, repo: Option<String>, repo_entry: Option<RepoEntries>) -> Result<String, ConfigError> {
    let mut x = String::new();

    File::open(get_root() + "/etc/mrp/config.json")
        .expect("failed to open the config file, pls check if a another process is accessing it or sum idk?")
        .read_to_string(&mut x)
        .expect("converting file to string failed")

    let config: Config = serde_json::from_str(&x).expect("date serialization failed");

    match entry {
        ConfigEntries::Architecture => ok(config.architecture),
        
    }
}