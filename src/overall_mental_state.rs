/// Overall Mental State tracking form`
/// ---
/// 
/// Mental state can be decribed with a range of emotions, feelings
/// so in a nutshell specific state can be presented as a function of 
/// different actions, feelings, emotions, mood collected over time
///
/// This form sets "Targets" for further implementation
/// Will be updated in the future
use chrono::{DateTime, Utc};


// This is an attempt to catch most desired states by providing a score or a binary answer
pub enum MentalStateEnum {
    // positive states
    motivation,  // the score can be turned into binary answer based on specific threshold
    calmness,
    creativity,
    flow,
    engagement,
    
    // negative states
    anxiety,
    fear,
    anhedonia,
    down,  // I would like to avoid "depressed" here since it might be mood, might be a clinical diagnosis which is out of target reach for this application
    disinterest
    stress,
}

pub struct Menatlstate {
    pub timestamp: DateTime<Utc>,
    pub state: HashMap<MentalStateEnum, i16>
}
