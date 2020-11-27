use chrono::{DateTime, Utc};
use strum_macros;
use crate::traits::SerializeInput;
use std::str::FromStr;
use strum::EnumString;
use serde::Serialize;


#[derive(Debug, strum_macros::ToString, EnumString, Clone, Serialize)]
#[serde(rename_all="lowercase")]
pub enum MedDosageUnit {
    #[strum(serialize="mg")]
    MG,
    #[strum(serialize="g")]
    G
}


#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct MedikamentationForm {
    pub name: String,
    pub dosage: i32,
    pub unit: MedDosageUnit,
    pub day_part: Option<String>,
    pub reason: Option<String>,
    pub side_effects: Option<String>,
    timestamp: String,  // TODO: write proprt setter to operate on private field in the code
}


impl FromStr for MedikamentationForm {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut args: Vec<String> = s.to_lowercase().split(',').map(|w| w.trim().to_string()).collect();
        if args.len() < 3 {
            panic!("Medical record should contain at least 3 non-empty strings devided by ','")
        };
        let mut side_effects = None;
        let mut reason = None;
        let mut day_part = None;
        let mut unit = MedDosageUnit::MG;
        let mut dosage = 0;
        let mut name = String::from("");
        match args.len() {
            // Is it better/faster to use several ifs and use pop -> go over all conditions?
            6 => {
                side_effects = args.pop();
                reason = args.pop();        
                day_part = args.pop();
            },
            5 => {
                reason = args.pop();        
                day_part = args.pop();
            },
            4 => {
                day_part = args.pop();
                unit = MedDosageUnit::from_str(&args.pop().unwrap()).unwrap();
                dosage = (args.pop().unwrap()).parse::<i32>().unwrap();
                name = args.pop().unwrap();
            },
            3 => {
                unit = MedDosageUnit::from_str(&args.pop().unwrap()).unwrap();
                dosage = (args.pop().unwrap()).parse::<i32>().unwrap();
                name = args.pop().unwrap();
            },
            _ => {panic!("The medical record is too short")}
        };
    

        Ok(MedikamentationForm{name, dosage, unit, day_part, reason, side_effects, timestamp: Utc::now().to_string()})
    }

}

impl SerializeInput for MedikamentationForm {
    fn to_vec(&self) -> Vec<String> {
        // prepare struct to be saved int csv file
        // TODO: use default trait?? THIS IS NOT THE WAY TO GO
        let day_part = self.day_part.clone().unwrap_or(String::from("")); 
        let reason = self.reason.clone().unwrap_or(String::from("")); 
        let side_effects = self.side_effects.clone().unwrap_or(String::from("")); 
        let resulting_vec = vec![self.name.clone(), 
        self.dosage.clone().to_string(), 
        self.unit.clone().to_string(), day_part, reason, side_effects, Utc::now().to_string()];
        resulting_vec
    }
}

pub fn show_info(s: &str) -> String {
    if s == "info" {
        format!("Available fields: name: str, dosage: number, unit: str (mg/g), (optional)time: str, (optinonal)reason: str, (optional)side_effects: str")
    }
    else if s == "example" {
        format!("Ibuprofen, 400, mg, evening, headache, drowsy")

    }
    else {
        panic!("unknown mode")
    }
}

