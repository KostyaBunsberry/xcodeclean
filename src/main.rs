use clap::Parser;
use std::fs::File;
use std::path::Path;
use dirs::home_dir;

// TODO: add simulator app deletion
/// Simple CLI to delete Xcode derived data and archives
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Should the program delete Xcode derived data
    #[arg(short)]
    derived_data_delete: bool,

    /// Should the program delete Xcode archives
    #[arg(short)]
    archives_data_delete: bool,
}

fn main() {
    let args = Args::parse();
    println!("Should delete derived data set to {}", &args.derived_data_delete);
    println!("Should delete archives set to {}", &args.archives_data_delete);

    if args.derived_data_delete {
        let homeDir = home_dir().expect("Home path could not be found");
        homeDir::push("Desktop");
        let is_derived_data_found = match Path::new("{}/Desktop/testDelete/DerivedData/").try_exists() {
            Ok(is_found) => is_found,
            Err(_) => {
                println!("Derived data directory was NOT found, try setting a custom path");
                return;
            }
        };
        println!("{}", is_derived_data_found);
    }
}
