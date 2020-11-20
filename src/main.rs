use structopt::StructOpt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;


//  mentalassist
// help: all functionality
// info: about this project and external links
// list:
//     --forms(?): show all available tracking forms and their short supposed usage
// track:
//     --type: Enumertaion for the available forms (type of tracking information) 
//     --file: str path to csv file where to APPEND the information to

#[derive(Debug, strum_macros::ToString)]
pub enum EntryType {
    #[strum(serialize="ActivityTracking: act")]
    Activity,
    #[strum(serialize="MedicationConsumption: med")]
    Medication
}



#[derive(Debug, StructOpt)]
#[structopt(name="Mental Assistant", version="0.0.1", 
            author="Alisa Dammer <alisa.dammer@gmail.com>", 
            about="Self tracking in various forms and for various purposes", 
            usage="trackme [SUBCOMMANDS] .. [OPTIONS] ..")]
enum Tracker {
    #[structopt(about="Prints out general information about this tool and available tracking approaches")]
    Info,
    #[structopt(about="Save tracking entiry of a specific type. See 'trackme list' for available tracking types")]
    Save {
        #[structopt(short="f", long="file")]
        file: String,
        #[structopt(short="et", long="entry_type")]
        entry_type: String// How to use enum here? EntryType
    },
    #[structopt(about="Show available tracking entry types")]
    List
}

fn main() {
    let args = Tracker::from_args(); // we can get multiple args later on, for now it is List anf Info
    println!("Args are {:?}", args);

    // all this code should be moved into separate module and properly encapsulated
    // Info: present information on main general info .txt
    //
    // List: show available forms (data entry types) that are defined in the code through EntryType
    //
    // Save: read in the user input, save it as a row in provided csv file (this should be
    // implemented after the data structures are defined, for now just return different wording of
    // the input)
    //
    match args {
        Tracker::List => println!("Available tracking data entries are: {:?}", format!("{}, {}", EntryType::Medication.to_string(), EntryType::Activity.to_string())),
        Tracker::Info => println!("Here the general info for the project and each tracking will be shown"),
        _ => println!("Still in construction"),
    }
}
