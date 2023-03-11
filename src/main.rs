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
        let mut path = home_dir().expect("Home path could not be found");
        path.push("Desktop");
        path.push("testDelete");
        let is_found = match path.as_path().try_exists() {
            Ok(is_found) => is_found,
            Err(_) => {
                println!("Error finding a path");
                return;
            }
        };
        println!("{}", is_found);
    }
}
