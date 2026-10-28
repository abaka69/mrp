use std::fmt::{self};
use serde::Deserialize;

pub struct ConfigError;

impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt:Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

pub enum ConfigEntries {
    Architecture,
    Toolchain,
    Color,
    Progressbar,
    Repos
}

pub enum RepoEntries {
    Name,
    Active,
    Url
}

#[derive(Deserialize)]
pub(super) struct Config {
    pub(super) architecture: String,
    pub(super) toolchain: String,
    pub(super) color: bool,
    pub(super) progressbar: bool,
    pub(super) repos: Vec<RepoNode>
}

#[derive(Deserialize)]
pub(super) struct RepoNode {
    pub(super) name: String,
    pub(super) active: bool,
    pub(super) url: Option<String>,
}