use chrono::{DateTime, Utc};
use strum_macros;
use crate::traits::SerializeInput;
use std::str::FromStr;
use serde::Serialize;

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
            _ => {panic!("{} is wrong length for base activity record expected 1 to 2 elements devided by ','", args.len())}
        };

        Ok(BaseActivity{name, category, timestamp: Utc::now().to_string()})
    }
}


impl SerializeInput for BaseActivity {
    fn to_vec(&self) -> Vec<String> {
        let resulting_vec = vec![self.name.clone(), self.category.clone().unwrap_or(String::from("")), Utc::now().to_string()];
        resulting_vec
    }
}


pub fn show_info(s: &str) -> String {
    if s == "info" {
        format!("Available fields: name: str, category: str")
    }
    else if s == "example" {
        format!("Pole dance, sport")

    }
    else {
        panic!("unknown mode")
    }
}
