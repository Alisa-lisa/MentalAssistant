#[warn(dead_code)]
pub mod meds_forms {

    #[derive(Debug)]
    pub enum MedDosageUnit {
        MG,
        G
    }

    #[derive(Debug)]
    pub struct MedikamentationForm {
        pub name: String,
        pub dosage: i32,
        pub unit: MedDosageUnit,
        pub day_part: Option<String>,
        pub reason: Option<String>,
        pub side_effects: Option<String>
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

} 


