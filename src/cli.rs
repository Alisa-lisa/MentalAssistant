use std::str::FromStr;
use structopt::StructOpt;
use strum_macros;
use std::error::Error;
use crate::meds;
use crate::activity;
use std::fs::OpenOptions;


#[derive(Debug, strum_macros::ToString, strum_macros::EnumIter)]
pub enum EntryType {
    #[strum(serialize="act")]
    Activity,
    #[strum(serialize="med")]
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
    Info 
}

impl FromStr for InformationMode {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "info" => Ok(InformationMode::Info),
            "example" => Ok(InformationMode::Example),
            _ => panic!("Unknown interaction mode"),
        }
    }
    
}


#[derive(Debug, StructOpt)]
#[structopt(name="Mental Assistant", version="0.0.1", 
            author, 
            about, 
            global_settings = &[structopt::clap::AppSettings::ColoredHelp],
            usage="trackme [SUBCOMMANDS] .. [OPTIONS] ..")]
pub enum Tracker {
    /// Prints out general information about this tool and available tracking approaches
    Info,
    /// Save data of a specific type. See 'trackme list' for available tracking types
    Save {
        /// File name, path to the file where to save the record
        #[structopt(short, long, help="If the file does not exist, it will be created")]
        file: String,
        /// Type of the tracking data to save
        #[structopt(short, long)]
        entrytype: EntryType, 
        /// Data to track structured accordingly to the chosen type 
        #[structopt(short, long)]
        data: String

    },
    /// Show available tracking entry types
    List,
    /// Show string structure of the specific entry type
    Show {
        /// Type of the tracking data to save
        #[structopt(short, long, help="use list command to get all available entry types")]
        entrytype: EntryType,
        /// which information to show: example or a structure of the record required
        #[structopt(short, long, help="use 'info' for structure of the record, 'example' for a valid example")]
        usage: InformationMode
    }
}


// save user input into providsed csv file accordingly to 
pub fn save_to_file(information_type: EntryType, input: &str, file_path: String) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)
        .unwrap();
    let mut wrt = csv::WriterBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_writer(file);
    
    match information_type {
        EntryType::Medication => {
           match meds::MedikamentationForm::from_str(input) {
                Ok(meds) => {
                    wrt.serialize(&meds)?;
                },
                Err(_) => {panic!("{:?} is not a valid input string for Medikamentation form", input);}
           }; 
        },
        EntryType::Activity => {
            match activity::BaseActivity::from_str(input) {
                Ok(activity) => {
                    wrt.serialize(&activity)?;
                },
                Err(_) => {panic!("{:?} is not a valid input string for Activity form", input);},
            }
        },
    };
    wrt.flush()?; 
    Ok(())
}  
