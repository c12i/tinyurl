use clap::{clap_app, crate_authors, crate_description, crate_name, crate_version};
use std::error::Error;
use std::process;
use tinyurl::tiny;

fn main() -> Result<(), Box<dyn Error>> {
    let clap = clap_app!(tinyurl =>
        (name:crate_name!())
        (about:crate_description!())
        (version:crate_version!())
        (author:crate_authors!())
        (@arg uri: +required "The uri to be shortened")
        (@arg alias: -a --alias +takes_value "Optional unique url alias")
    )
    .get_matches();

    match (clap.value_of("uri"), clap.value_of("alias")) {
        (Some(uri), None) => {
            if let Ok(short) = tiny!(uri) {
                if short == "Error" {
                    println!("The url you entered is invalid");
                    process::exit(1);
                }
                println!("{}", short);
            }
        }
        (Some(uri), Some(alias)) => {
            if let Ok(short) = tiny!(uri, alias = alias) {
                if short == "Error" {
                    println!("The url/ alias you entered is invalid. Note that the alias must be at least 5 characters long");
                    process::exit(1);
                }
                println!("{}", short);
            }
        }
        _ => println!("Something went wrong..."),
    }

    Ok(())
}
