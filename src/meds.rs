use chrono::{DateTime, Utc};
use strum_macros;

/// # Medication Tracking
///
/// simple way to collect basic information about the meds/suppliments/etc  
///
/// TODO: comments can belong to tags like "sideeffect", "reason", "etc" 
///
///

#[derive(strum_macros::ToString, Debug)]
pub enum Unit {
    #[strum(serialize = "mg")]
    Mg, 
    #[strum(serialize = "g")]
    G, 
    #[strum(serialize = "ml")]
    Ml, 
    #[strum(serialize = "l")]
    L
}


pub struct Medication {
    pub name: String,
    pub dosage: i16,
    pub unit: Unit,
    pub timestamp: DateTime<Utc>,
    pub comment: Option<String>  // TODO: set tagging for comments: Tag::SideEffect, "headache" or Tag::Reason, "prescription"  
}

impl Medication {
    /// Constructor method 
    // TODO: do i need to create 2 constructors to ignore comment completely?
    pub fn new(name: String, dosage: i16, unit: Unit, timestamp: DateTime<Utc>, comment: Option<String>) -> Medication {
        let res = Medication{name, dosage, unit, timestamp, comment};
        res
    }
        
}


impl ToString for Medication {
    fn to_string(&self) -> String {
        let res: String = format!("name: {}, dosage: {}, unit: {}, timestamp: {}", 
                                  &self.name.clone(), &self.dosage, 
                                  &self.unit.to_string(), &self.timestamp.to_string());
        res
    }
}



#[test]
fn positive_to_string() {
    let today = Utc::now();
    let medication = Medication::new(String::from("Aspirin"), 200, Unit::Mg, today, None);
    let expcted: String = String::from(format!("name: Aspirin, dosage: 200, unit: mg, timestamp: {}", today.to_string()));
    assert_eq!(medication.to_string(), expcted); 

}

