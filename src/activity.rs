/// Activity tracking
/// ---
///
/// Basic struct with fields describing a simple activity fr further analysis

use chrono::{DateTime, Utc};


pub struct Activity {
    pub timestamp: DateTime<Utc>,
    pub activity: String,
    pub category: Option<String>,
    pub comment: Option<String>  // This struct can be extended in the future with associatyed emotions and other tags from "mental balance" topic
} 

impl Activity {
    /// Constructor
    pub fn new(timestamp: Option<DateTime<Utc>>, activity: String, category: Option<String>, comment: Option<String>) -> Activity {
        let mut time = Utc::now();
        match timestamp.is_some() {
            true => { time = timestamp.unwrap();},
            false => {}
        };
        let res = Activity{timestamp: time, activity, category, comment};
        res
    }
}

impl ToString for Activity {
    fn to_string(&self) -> String {
        let res: String = format!("On {} {} was done. It was {}", &self.timestamp.date().to_string(), &self.activity, 
                                  &self.comment.clone().unwrap_or(String::from("Ok")));
        res
    }
}


#[test]
fn to_string() {
    let today = Utc::now().date();
    let activity = Activity::new(None, String::from("Sport"), None, None);

    assert_eq!(activity.to_string(), format!("On {} Sport was done. It was Ok", today.to_string()));
}
