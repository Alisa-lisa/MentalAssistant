pub mod meds_forms {

    #[derive(Debug)]
    pub enum MedDosageUnit {
        MG = 1,
        G = 1000
    }

    #[derive(Debug)]
    pub enum DayPart {
        MORNING = 1,
        FirstHalfDay = 2,
        SecondHalfDay = 3,
        EVENING = 4,
        NIGHT = 5
    }
    
    #[derive(Debug)]
    pub struct MedikamentationForm {
        pub name: String,
        pub dosage: i32,
        pub unit: MedDosageUnit,
        pub day_part: Option<DayPart>,
        pub reason: Option<String>,
        pub side_effects: Option<String>
    }
    
} 
