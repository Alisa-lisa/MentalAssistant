use std::str::FromStr;
use structopt::StructOpt;
use strum_macros;
use strum::IntoEnumIterator;

mod meds;
use meds::meds_forms;

//  mentalassist
// help: all functionality
// info: about this project and external links
// list:
//     --forms(?): show all available tracking forms and their short supposed usage
// track:
//     --type: Enumertaion for the available forms (type of tracking information) 
//     --file: str path to csv file where to APPEND the information to

#[derive(Debug, strum_macros::ToString, strum_macros::EnumIter)]
pub enum EntryType {
    #[strum(serialize="ActivityTracking: act ")]
    Activity,
    #[strum(serialize="MedicationConsumption: med ")]
    Medication
}

impl FromStr for EntryType {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed_string = s.trim();
        match parsed_string {
            "act" => Ok(EntryType::Activity),
            "med" => Ok(EntryType::Medication),
            _ => panic!("Unidentified variant of EntryType enum"),
        }

    }
}

#[derive(Debug, strum_macros::ToString, strum_macros::EnumIter)]
pub enum InformationMode {
    #[strum(serialize="example")]
    Example,
    #[strum(serialize="info")]
    Strcture 
}


impl FromStr for InformationMode {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "info" => Ok(InformationMode::Strcture),
            "example" => Ok(InformationMode::Example),
            _ => panic!("Unknown intarction mode"),
        }
    }
    
}

#[derive(Debug, StructOpt)]
#[structopt(name="Mental Assistant", version="0.0.1", 
            author, 
            about, 
            global_settings = &[structopt::clap::AppSettings::ColoredHelp],
            usage="trackme [SUBCOMMANDS] .. [OPTIONS] ..")]
enum Tracker {
    /// Prints out general information about this tool and available tracking approaches
    Info,
    /// Save tracking entiry of a specific type. See 'trackme list' for available tracking types
    Save {
        #[structopt(short, long)]
        file: String,
        #[structopt(short, long)]
        entrytype: EntryType, 
        #[structopt(short, long)]
        data: String

    },
    /// Show available tracking entry types
    List,
    /// Show string structure of the specific entry type
    Show {
        #[structopt(short, long)]
        entrytype: EntryType,
        #[structopt(long)]
        usage: InformationMode
    }
}



fn main() {
    let args = Tracker::from_args(); // we can get multiple args later on, for now it is List anf Info
    // all this code should be moved into separate module and properly encapsulated
    // Info: present information on main general info .txt
    //
    // List: show available forms (data entry types) that are defined in the code through EntryType
    // Show + type of data entry: show how a string should look like to pe parsable
    // Save: read in the user input, save it as a row in provided csv file (this should be
    // implemented after the data structures are defined, for now just return different wording of
    // the input)
    //
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
