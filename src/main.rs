use clap::Parser;
use dirs::home_dir;
use std::{fs::{remove_dir_all, create_dir}, path::{Path, PathBuf}};

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

    /// Sets custom path to Developer folder (saves between sessions), use pwd in your Developer folder to set this argument
    #[arg(long="set-custom-path")]
    custom_path: Option<String>, // use this path if set

    /// Resets path to default
    #[arg(long="set-default-path")]
    should_delete_custom_path: bool,
}

#[derive(Savefile)]
struct CustomPath {
    path_string: String
}

fn save_custom_path(path:CustomPath) {
    save_file("xcodeclean_save.bin", 0, &path).unwrap();
    println!("✅ Custom path was set")
}

fn remove_custom_path() {
    match std::fs::remove_file("xcodeclean_save.bin") {
        Ok(_) => {
            println!("✅ Custom path was removed")
        },
        Err(_) => {
            println!("❌ Custom path couldn't be removed, maybe no path was set")
        }
    };
}

fn load_custom_path() -> Result<CustomPath, SavefileError> {
    load_file("xcodeclean_save.bin", 0)
}

fn developer_path() -> Option<Box<PathBuf>> {
    match load_custom_path() {
        Ok(custom_path) => {
            let path = Path::new(&custom_path.path_string);
            let path_buf = PathBuf::from(path);
            return Some(Box::new(path_buf));
        },
        Err(_) => {
            let mut default_dev_path = match home_dir() {
                Some(home_path) => home_path,
                None => {
                    println!("❌ $HOME path could not be found");
                    return None;
                },
            };
            default_dev_path.push("Library/Developer/Xcode/");
            return Some(Box::new(default_dev_path));
        }
    }
}

fn delete_derived_data() {
    let mut derived_path = developer_path().expect("Default path could not be found or custom path could not be parsed");
    derived_path.push("DerivedData");

    if !match derived_path.as_path().try_exists() {
            Ok(is_found) => is_found,
            Err(_) => {
                println!("❌ Error finding archives folder");
                return;
            }
        } {
        println!("❌ DerivedData folder was not found at {}", derived_path.as_path().display());
        println!("🧐 If you have a custom path for Developer folder set a custom path with --set-custom-path, or remove custom path with --set-default-path");
        return;
    }
    
    match remove_dir_all(derived_path.as_path()) {
        Ok(_) => {
          match create_dir(derived_path.as_path()) {
            Ok(_) => println!("✅ DerivedData folder is now empty!"),
            Err(_) => println!("❌ Error re-creating DerivedData folder")
          }; 
        },
        Err(_) => println!("❌ Error deleting DerivedData folder")
    };
}

fn delete_archives() {
    let mut archives_path = developer_path().expect("Default path could not be found or custom path could not be parsed");
    archives_path.push("Archives");

    if !match archives_path.as_path().try_exists() {
            Ok(is_found) => is_found,
            Err(_) => {
                println!("❌ Error finding archives folder");
                return;
            }
        } {
        println!("❌ Archives folder was not found");
        println!("🧐 If you have a custom path for Developer folder set a custom path with --set-custom-path, or remove custom path with --set-default-path");
        return;
    }

    match remove_dir_all(archives_path.as_path()) {
        Ok(_) => {
          match create_dir(archives_path.as_path()) {
            Ok(_) => println!("✅ Archives folder is now empty!"),
            Err(_) => println!("❌ Error re-creating archives folder")
          }; 
        },
        Err(_) => println!("❌ Error deleting archives folder")
    };
}

fn main() {
    let args = Args::parse();

    match args.custom_path {
        Some(path_string) => {
            let custom_path = CustomPath {
                path_string: path_string
            };
            if !args.should_delete_custom_path {
                save_custom_path(custom_path);
            } else {
                println!("🧐 You have set custom path to remove and then asked to set it... Bipolar much? I'm gonna remove it")
            }
        },
        None => {
            if !args.should_delete_derived && !args.should_delete_archives && !args.should_delete_custom_path {
                println!("No arguments were set, use --help to see options")
            }
        }
    }

    if args.should_delete_custom_path {
        remove_custom_path();
    }

    if args.should_delete_derived {
        delete_derived_data();
    }
    
    if args.should_delete_archives {
        delete_archives();
    }
}
