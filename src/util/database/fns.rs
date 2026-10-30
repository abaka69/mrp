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

