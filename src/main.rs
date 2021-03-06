mod meds;
mod activity;
mod cli;
use cli::{Tracker, EntryType, save_to_file};
use strum::IntoEnumIterator;
use structopt::StructOpt;
use std::process;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let args = Tracker::from_args();     
    match args {
        Tracker::List => {
            let mut available_forms = String::from("Available tracking forms are: ");
            for t in EntryType::iter() {
                available_forms.push_str(&t.to_string());
            }
            println!("{}", available_forms);
        },
        Tracker::Info => {
            println!("Here the general info for the project and each tracking will be shown");
        },
        Tracker::Save{file, entrytype, data} => {
            if let Err(err) = save_to_file(entrytype, &data, file) {
                println!("Could not save entry due to {}", err);
                process::exit(1);
            }
        },
        Tracker::Show{entrytype, usage} => {
            match entrytype {
                EntryType::Medication => {
                    println!("{}", meds::show_info(&usage));
                },
                EntryType::Activity => {
                    println!("{}", activity::show_info(&usage));
                },
            }
        }
    }
    Ok(())
}
