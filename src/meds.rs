use chrono::{DateTime, Utc};
use strum_macros;
use std::str::FromStr;
use strum::EnumString;
use serde::Serialize;
use crate::cli::InformationMode;


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


pub fn show_info(s: &InformationMode) -> String {
    let mut res = String::new();
    match s {
        &InformationMode::Info => {
            res = String::from("Available fields: name: str, dosage: number, unit: str (mg/g), (optional)time: str, (optinonal)reason: str, (optional)side_effects: str");
        },
        &InformationMode::Example => {
            res = String::from("Ibuprofen, 400, mg, evening, headache, drowsy");
        }
    };
    res
}

