use structopt::StructOpt;
use std::io;


//  mentalassist
// help: all functionality
// info: about this project and external links
// list:
//     --forms(?): show all available tracking forms and their short supposed usage
// track:
//     --type: Enumertaion for the available forms (type of tracking information) 
//     --file: str path to csv file where to APPEND the information to

#[derive(Debug)]
enum EntryType {
    Activity(String),
    Medication(String)
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
    let args = Tracker::from_args();
    println!("Args are {:?}", args);

}
