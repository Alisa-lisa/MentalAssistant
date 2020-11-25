use chrono::{DateTime, Utc};
use strum_macros;
use crate::traits::SerializeInput;


#[derive(Debug, strum_macros::ToString, Copy, Clone)]
pub enum MedDosageUnit {
    #[strum(serialize="mg")]
    MG,
    #[strum(serialize="g")]
    G
}

#[derive(Debug)]
pub struct MedikamentationForm {
    pub name: String,
    pub dosage: i32,
    pub unit: MedDosageUnit,
    pub day_part: Option<String>,
    pub reason: Option<String>,
    pub side_effects: Option<String>,
    pub timestamp: DateTime<Utc>,  // TODO: write proprt setter to operate on private field in the code
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

