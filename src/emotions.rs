/// Mental state tracking form
/// ---
///
/// Currently implemented fields are primary classes of emotions or so called "primary emotions".
/// These emotions are normally very strong and obvious, additionally since there are so many
/// secondary and tertiary emotions a list with free input is available.
/// It is planned to add the list of secondary emotions to the project as a orientation entity.
///
///
/// It is NOT ICD-9-CM compliant.
use chrono::{DateTime, Utc};
use std::collections::HashMap;


pub struct Emotions {
    pub timestamp: DateTime<Utc>,
    pub anger: i16,  // How angry do you feel on the scale X..Y
    pub sadness: i16,  // How sad do you feel on the scale X..Y
    pub fear: i16,  // How fearful are you? Anxious, nervous, etc.
    pub happiness: i16,  // How joyful are you
    pub surprise: u16,  // Astonishment, amuzement, wonder - can be both positive and negative, thus unsigned 
    pub disgust: i16,  // What makes you keep distance 
    pub shame: i16,
    pub secondary: HashMap<String, i16>  // Name of the emotions outside of primary emotions 
}

impl Emotions {
}

impl ToString for Emotions {
    fn to_string(&self) -> String {
        String::from(format!("On {} scored anger: {}, sadness: {}, fear: {}, happiness: {}, surprise: {}, disgust: {}, shame: {}", 
                             &self.timestamp.date().to_string(), &self.anger, 
                             &self.sadness, &self.fear,
                             &self.happiness, &self.surprise,
                             &self.disgust, &self.shame))
    }
}


#[test]
fn to_string() {
    let today = Utc::now().date().to_string();
    let secondary: HashMap<String, i16> = HashMap::new();
    let emotions = Emotions{timestamp: Utc::now(), anger: 0, sadness: 2, fear: 2, 
        happiness: 4, surprise: 0, disgust: 2, shame: 1, secondary};

    assert_eq!(emotions.to_string(), format!("On {} scored anger: 0, sadness: 2, fear: 2, happiness: 4, surprise: 0, disgust: 2, shame: 1", today));
}
