use chrono::{DateTime, Utc};
use strum_macros;
use crate::traits::SerializeInput;


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
    pub category: Option<String>, // TODO extend this to show also custom saved categories
    pub timestamp: DateTime<Utc>,
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
