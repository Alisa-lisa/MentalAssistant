use std::str::FromStr;
use structopt::StructOpt;
use strum_macros;
use std::error::Error;
use crate::traits::SerializeInput;
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
pub enum Tracker {
    /// Prints out general information about this tool and available tracking approaches
    Info,
    /// Save tracking entiry of a specific type. See 'trackme list' for available tracking types
    Save {
        #[structopt(short, long, help="If the file does not exist, it will be created")]
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
        #[structopt(short, long, help="use list command to get all available entry types")]
        entrytype: EntryType,
        #[structopt(short, long, help="use 'info' for structure of the record, 'example' for a valid example")]
        usage: InformationMode
    }
}

pub fn prepare_record(information_type: EntryType, input: &str, record: &mut Vec<String>) {
    let res;
    match information_type {
        // TODO: propagate error meessage from trait implementation
        EntryType::Activity => {
            match activity::BaseActivity::from_str(input) {
                Ok(activity) => {
                    res = activity.to_vec();
                },
                Err(_) => {panic!("{:?} is not a valid input string for Activity form", input)}
            }
        },
        EntryType::Medication => {
            // handle missing values + handle proper string validation -> serde?
            match meds::MedikamentationForm::from_str(input) {
                Ok(meds) => {
                        res = meds.to_vec();
                },
                Err(_) => {panic!("{:?} is not a valid input string for Medikamentation form", input);
                }
            }
        },
    }
    record.extend(res);
} 


// save user input into providsed csv file accordingly to 
pub fn save_to_file(information_type: EntryType, input: &str, file_path: String) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)
        .unwrap();
    let mut wrt = csv::Writer::from_writer(file);
    let mut record = Vec::new();
    record.push(information_type.to_string());
    prepare_record(information_type, input, &mut record);
    // construct record and write it into writer
    wrt.write_record(&record);
    wrt.flush()?; 
    Ok(())
}  
