use clap::Parser;
use dirs::home_dir;
use std::fs::{remove_dir_all, create_dir};

extern crate savefile;
use savefile::prelude::*;

#[macro_use]
extern crate savefile_derive;

// TODO: add simulator app deletion
/// Simple CLI to delete Xcode derived data and archives
#[derive(Parser, Debug)]
#[command(author="Kostya Bunsberry", version, about, long_about = None)]
struct Args {
    /// Should the program delete Xcode derived data
    #[arg(short='d')]
    should_delete_derived: bool,

    /// Should the program delete Xcode archives
    #[arg(short='a')]
    should_delete_archives: bool,

    /// Sets custom path to Developer folder
    #[arg(long="set-custom-path")]
    custom_path: Option<String>, // use this path if set
}

#[derive(Savefile)]
struct CustomPath {
    path_string: String
}

fn save_custom_path(path:&CustomPath) {
    save_file("save.bin", 0, path).unwrap();
}

fn load_custom_path() -> CustomPath {
    load_file("save.bin", 0).unwrap()
}

fn delete_derived_data() {
    let mut derived_path = match home_dir() {
        Some(home_path) => home_path,
        None => {
            println!("❌ $HOME path could not be found");
            return;
        },
    };
    derived_path.push("Library/Developer/Xcode/DerivedData");

    if !match derived_path.as_path().try_exists() {
            Ok(is_found) => is_found,
            Err(_) => {
                println!("❌ Error finding archives folder");
                return;
            }
        } {
        println!("❌ DerivedData folder was not found");
        println!("🧐 If you have a custom path for Developer folder set a custom path with --set-custom-path");
        return;
    }
    
    remove_dir_all(derived_path.as_path());
    create_dir(derived_path.as_path());
    println!("✅ DerivedData folder is now empty!");
}

fn delete_archives() {
    let mut archives_path = match home_dir() {
        Some(home_path) => home_path,
        None => {
            println!("❌ $HOME path could not be found");
            return;
        },
    };
    archives_path.push("Library/Developer/Xcode/Archives");

    if !match archives_path.as_path().try_exists() {
            Ok(is_found) => is_found,
            Err(_) => {
                println!("❌ Error finding archives folder");
                return;
            }
        } {
        println!("❌ Archives folder was not found");
        println!("🧐 If you have a custom path for Developer folder set a custom path with --set-custom-path");
        return;
    }

    remove_dir_all(archives_path.as_path());
    create_dir(archives_path.as_path());
    println!("✅ Archives folder is now empty!")
}

fn main() {
    let args = Args::parse();

    if args.should_delete_derived {
        delete_derived_data();
    }
    
    if args.should_delete_archives {
        delete_archives();
    }
}
