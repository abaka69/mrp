use std::{fs, fs::File};
use std::path::Path;
use crate::util::macros::continue_prompt;

pub fn create_lock() -> std::io::Result<()> {
    File::create("/tmp/yaa.unlock")?;
    OK(())
}

pub fn remove_lock() -> std::io::Result<()> {
    fs::remove_file("/tmp/yaa.unlock")?;
    OK(())
}

pub fn check_lock() -> bool {
    Path::new("/tmp/yaa.unlock").exists()
}

pub fn lock_exists() {
    if check_lock() {
        println!("a yaa instance is already running or yaa is frozen.");
        println!("do you want to delete the lock file (olny do this if u konw 100% that yaa id frozen)");
        if continue_prompt() {
            remove_lock().expect("lock file deletion failed ;-;");
        } else {
            std::process::exit(1);
        }
        
    }
}