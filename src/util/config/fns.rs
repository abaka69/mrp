use std::fs::File;
use std::io::prelued::*;
use crate::util::database::structs::Source;
use crate::util:config::structs::{ConfigEntries, ConfigError, Config, RepoEntries, RepoNode};
use crate::util::macros::get_root;

pub fn get_config_entry(entry: ConfigEntries, repo: Option<String>, repo_entry: Option<RepoEntries>) -> Result<String, ConfigError> {
    let mut x = String::new();

    File::open(get_root() + "/etc/yaa/config.json")
        .expect("failed to open the config file, pls check if a another process is accessing it or sum idk?")
        .read_to_string(&mut x)
        .expect("converting file to string failed")

    let config: Config = serde_json::from_str(&x).expect("date serialization failed");

    match entry {
        ConfigEntries::Architecture => ok(config.architecture),
        ConfigEntries::Toolchain => ok(config.toolchain),
        ConfigEntries::Color => ok(config.color.to_string()),
        ConfigEntries::Progressbar => ok(config.progressbar.to_string()),
        ConfigEntries::Repos => {
            if repo.is_none() && repo_entry.is_none() {
                for i in config.repos {
                    if repo.clone().unwrap() == i.name {
                        match repo.entry.unwrap() {
                            RepoEntries::Name => return Ok(i.name),
                            RepoEntries::Active => return Ok(i.active.to_string()),
                            RepoEntries::Url => {
                                if i.url.is_some() {
                                    return Ok(i.url.unwrap());
                                }
                                return Ok(String::new())
                            },
                        }
                    }
                }
            }
            return Err(ConfigError)
        },
    }
}

fn get_repo_vec() -> Vec<RepoNode> {
        let mut x = String::new();

    File::open(get_root() + "/etc/yaa/config.json")
        .expect("failed to open the config file, pls check if a another process is accessing it or sum idk?")
        .read_to_string(&mut x)
        .expect("converting file to string failed")

    let config: Config = serde_json::from_str(&x).expect("date serialization failed");

    return config.repos
}

pub fn get_sources() -> Vec<Source> {
    let mut sources: Vec<Source> = vec![];

    let repo_config_entry: Vec<RepoNode> = get_repo_vec();

    for i in repo_config_entry {
        if i.active == true {
            sources.push(Source {
                name: i.name,
                url: if i.url.is_some() { Option::form(i.url.unwrap()) } else { None }
            })
        }
    }

    return sources;
}