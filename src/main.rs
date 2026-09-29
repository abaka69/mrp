use std::env;

mod commands;
mod util;

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        commands::help::help();
        std::process::exit(0);
    }

    let command: String = args[1].to_lowercase();

    match &command[..] {

        "-h" => commands::help::help(),
        "--help" => commands::help::help(),
        
        "-s" => commands::sync::sync(),
        "--sync" => commands::sync::sync(),

        "-u" => commands::upgrade::upgrade(),
        "--update" => commands::upgrade::upgrade(),

        "-i" => commands::install::install(),
        "--install" => commands::install::install(),
        "-il" => commands::installlocal::installlocal(),
        "--installlocal" => commands::installlocal::installlocal(),
        "-ig" => commands::installgroup::installgroup(),
        "--installgroup" => commands::installgroup::installgroup(),

        "-r" => commands::remove::remove(),
        "--remove" => commands::remove::remove(),

        "--list" => commands::list::list(),
        
        _ => {
            println!("mrp: woopsies bad command: \"{}\", pls use {{-h --help}} for good commands OwO", command);
        }
    }
}