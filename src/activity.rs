use chrono::{DateTime, Utc};
use std::str::FromStr;
use structopt::StructOpt;
use strum_macros;

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


#[derive(Debug)]
pub struct BaseActivity {
    pub name: String,
    pub category: Option<BaseActivityCategories>, // TODO extend this to show also custom saved categories
    timestamp: DateTime<Utc>,
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
