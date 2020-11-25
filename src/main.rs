mod meds;
mod traits;
mod activity;
mod cli;
use cli::{Tracker, EntryType, save_to_file};
use strum::IntoEnumIterator;
use structopt::StructOpt;
use std::process;

fn main() {
    let args = Tracker::from_args();     
    match args {
        Tracker::List => {
            let mut available_forms = String::from("Available tracking forms are: ");
            for t in EntryType::iter() {
                available_forms.push_str(&t.to_string());
            }
            println!("{}", available_forms);
        },
        Tracker::Info => println!("Here the general info for the project and each tracking will be shown"),
        Tracker::Save{file, entrytype, data} => {
            if let Err(err) = save_to_file(entrytype, &data, file) {
                println!("{}", err);
                process::exit(1);
            }
        },
        Tracker::Show{entrytype, usage} => {
            match entrytype {
                EntryType::Medication => {
                   println!("{:?}", meds::show_info(&usage.to_string()));
                },
                EntryType::Activity => {
                    println!("{:?}", activity::show_info(&usage.to_string()));
                },
            }
        }
    }

}
