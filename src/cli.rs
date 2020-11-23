use std::str::FromStr;
use structopt::StructOpt;
use strum_macros;


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
pub enum Tracker {
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
        #[structopt(short, long, help="use list command to get all available entry types")]
        entrytype: EntryType,
        #[structopt(short, long, help="use 'info' for structure of the record, 'example' for a valid example")]
        usage: InformationMode
    }
}

