use std::env;

mod commands;
mod util;

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

