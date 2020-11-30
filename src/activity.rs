use chrono::{DateTime, Utc};
use strum_macros;
use std::str::FromStr;
use serde::Serialize;
use crate::cli::InformationMode;


// Here i need to save custom categories and show them as well later on with the predefined ones
#[derive(Debug, strum_macros::ToString, strum_macros::EnumIter)]
pub enum BaseActivityCategories {
    Sport,
    Relax,
    PersonalCare,
    Shopping,
    Education,
    Work,
    Movies,
    Social,
    Reading,
    Travel,
    EatingOut,
    Craft,
    Cleaning,
    Chores
}


#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct BaseActivity {
    pub name: String,
    pub category: Option<String>, // TODO extend this to show also custom saved categories
    timestamp: String,
}

impl FromStr for BaseActivity {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut args: Vec<String> = s.to_lowercase().split(',').map(|w| w.trim().to_string()).collect();
        let name;
        let mut category = None;
        match args.len() {
            2 => {
                category = args.pop();
                name = args.pop().unwrap();
            },
            1 => {
                name = args.pop().unwrap();
            },
            _ => { 
                println!("Unexpected length of activity string. Got {} eleemnts devided by ','. String should have 1-2 words devided by coma", args.len());
                std::process::exit(1);
            }
        };

        Ok(BaseActivity{name, category, timestamp: Utc::now().to_string()})
    }
}


pub fn show_info(s: &InformationMode) -> String {
    let mut res = String::new();
    match s {
        &InformationMode::Info => {
            res = String::from("Available fields: name: str, category: str");
        },
        &InformationMode::Example => {
            res = String::from("Pole dance, sport");
        }
    };
    res
}
