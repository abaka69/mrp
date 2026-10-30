use std::fs::{self, File};
use std::path::Path;

use tar::Archive;
use xz2::read::XzDecoder;

use crate::utils::{database::fns::{remove_package_from_installed, return_owned_files}, packaging::structs::Package};

pub fn decompress_xz(compressed_tar: File) -> Archive<XzDecoder<File>> {
    return Archive::new(XzDecoder::new(compressed_tar));
}

pub fn decode_pkg_file(pkg: File) -> Package {
    let v: Package = serde_json::from_reader(pkg).unwrap();

    return v;
}

pub fn check_if_package(mut xztar: Archive<XzDecoder<File>>) -> bool {
    for file in xztar.entries().unwrap() == Path::new("PKG") {
        if file.unwrap().header().path().unwrap() == Path::new("PKG") {
            return true;
        }
    }

    return false;
}

pub fn run_remove(package: &String) {
    for x in return_owned_files(package).expect("failed to get owned files?") {
        if Path::new(&x).exists() {
            fs::remove_file(x).expect("failed to remove files?")
        }
    }

    remove_package_from_installed(package).expect("failed tp remove package from db?");
    
}