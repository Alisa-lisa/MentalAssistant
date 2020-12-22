use chrono::{DateTime, Utc, NaiveDateTime};


/// # Sleep tracking
/// ---
///
/// this is a general form to store data the way it will be easily comapitble 
/// with other data sources for further analysis
///

pub struct Sleep {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub comments: Option<String>,  // comma separated comments TODO: later to use tags: sleepquality, dreamtype
    pub evening_energy_score: i16, // the scale is not limited in this struct, but should be set based on the later functionality implementation
    pub morning_energy_score: i16
}

impl Sleep {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>, comments: Option<String>, evening_energy: i16, morning_energy: i16) -> Sleep {
        Sleep{start, end, comments, evening_energy_score: evening_energy, morning_energy_score: morning_energy}
    }
}

impl ToString for Sleep {
    fn to_string(&self) -> String {
        let commentary: String = self.comments.clone().unwrap_or(String::from("Ok"));
        let res: String = format!("Sleeping between {:?} and {:?}, was: {}", &self.start.date().naive_utc(), &self.end.date().naive_utc(), commentary);
        res
    }
}


#[test]
fn to_string() {
    // TODO: how to mock time in order ro use DateTime::now() freely
    let start: DateTime<Utc> = DateTime::parse_from_rfc3339("2020-12-21T23:40:00+02:00").unwrap().with_timezone(&Utc);
    let end: DateTime<Utc> = DateTime::parse_from_rfc3339("2020-12-22T08:12:00+02:00").unwrap().with_timezone(&Utc);
    let sleep = Sleep::new(start, end, None, 20, 67);
    
    assert_eq!(sleep.to_string(), "Sleeping between 2020-12-21 and 2020-12-22, was: Ok");
}
