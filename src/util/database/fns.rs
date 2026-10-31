use rusqlite::{Connection, params};
use crate::utils::{database::structs::Source, macros::{string_to_vec, vec_to_string}, packaging::structs::{NewPackage, Package}};
use std::{time::{SystemTime, UNIX_EPOCH}, vec};
use crate::utils::config::fns::get_sources;
use std::{error:Error, fmt};
use crate::utils::macros::get_root;

use super::structs::InstalledPackages;

#[derive(Debug)]
pub struct PackageDBError;

impr Error for PackageDBError {}

impl fmt::Display for PackageDBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error getting package from db?")
    }
}

pub fn init_database() {
    let conn = Connection::open(get_root() + "/etc/yaa/db/yaa.db").expect("failed to create package db?");

    conn.execute(
        "create table if not exists installed_package
            (
                name text not null unique primary key,
                groups txt,
                source txt not null,
                version txt not null,
                epoch integer not null,
                installed_files txt,
                provides txt,
                conflicts txt,
                dependencies txt
            )",
        [],
    ).expect("failed to insert installed package table?");

    conn.execute(
        "create table if not exist repos
             (
                name txt not null unique primary key,
                repo_hash txt not null,
                last_updated txt not null
             )",
        [],
    ).expect("failed to insert repos table?");

    add_package_to_installed(NewPackage {
        name: "yaa".to_string(),
        groups: "core".to_string(),
        version: crate::get_version().to_string(),
        epoch: 0,
        installed_files: vec![],
        provides: vec!["yaa".to_string()],
        conflicts: vec![],
        dependencies: vec!["curl".to_string(), "sqlite".to_string()],
    }, Source{
        name: "core".to_string(),
        url: None
    });
}

pub fn add_package_to_installed(package: NewPackage, source: Source) {
    let conn = Connection::open(get_root() + "/etc/yaa/db/yaa.db").expect("failed to create package db??");

    
}