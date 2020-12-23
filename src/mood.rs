/// Mood tracking form
/// ---
///
/// Basic fields allowing to use mood entries in further analysis
use chrono::{DateTime, Utc};


pub struct Mood {
    pub timestamp: DateTime<Utc>,
    pub score: i16,  // the scale can be different depending on the actual implementation 0-10 is recommended
    pub comment: Option<String>,  // TODO: add tags in the future describing mood
    pub associated_color: Option<String>  // this one is funny, since it might be more accurate than the actual words + it is a calibration method of the chosem score scala
}

impl Mood {
    /// Constructor mehtod
    pub fn new(timestamp: DateTime<Utc>, score: i16, comment: Option<String>, associated_color: Option<String>) -> Mood {
        let res = Mood{timestamp, score, comment, associated_color};
        res
    }

    
}

impl ToString for Mood {
    fn to_string(&self) -> String {
        let res: String = format!("On {} my mood was {}, felt like {}", &self.timestamp.date().to_string(), 
                                  &self.score, self.associated_color.clone().unwrap_or(String::from("Undefined")));
        res
    }
}


#[test]
fn to_string() {
    let mood = Mood::new(Utc::now(), 7, None, None);
    let mood1 = Mood::new(Utc::now(), 6, Some(String::from("weird, tired, okish")), Some(String::from("Green")));

    assert_eq!(mood.to_string(), String::from(format!("On {} my mood was 7, felt like Undefined", Utc::now().date())));
    assert_eq!(mood1.to_string(), String::from(format!("On {} my mood was 6, felt like Green", Utc::now().date())));
}
