mod meds;
mod cli;
use meds::meds_forms;
use cli::{Tracker, EntryType};
use strum::IntoEnumIterator;
use structopt::StructOpt;


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
            println!("Saving data {:?} of type {:?} to file {:?}", data, entrytype, file);
        },
        Tracker::Show{entrytype, usage} => {
            match entrytype {
                EntryType::Medication => {
                   println!("{:?}", meds_forms::show_info(&usage.to_string()));
                },
                EntryType::Activity => println!("Under construction"),
            }
        }
    }

}
