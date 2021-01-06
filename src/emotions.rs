/// Mental state tracking form
/// ---
///
/// Currently implemented fields are primary classes of emotions or so called "primary emotions".
/// These emotions are normally very strong and obvious, additionally since there are so many
/// secondary and tertiary emotions a list with free input is available.
/// It is planned to add the list of secondary emotions to the project as a orientation entity.
/// Parrot's emotions tree structure: [wiki](!https://en.wikipedia.org/wiki/Emotion_classification) 
///
use chrono::{DateTime, Utc};
use std::collections::HashMap;


pub struct Emotions {
    pub timestamp: DateTime<Utc>,
    pub anger: i16,  // How angry do you feel on the scale X..Y
    pub sadness: i16,  // How sad do you feel on the scale X..Y
    pub fear: i16,  // How fearful are you? Anxious, nervous, etc.
    pub joy: i16,  // How joyful are you
    pub surprise: u16,  // Astonishment, amuzement, wonder - can be both positive and negative, thus unsigned 
    pub love: i16,  // What makes you keep distance 
    pub secondary: HashMap<SecondaryAndTertiaryEmotions, i16>,  // Name of the emotions outside of primary emotions 
    pub comment: Option<String>
}

impl Emotions {
}

impl ToString for Emotions {
    fn to_string(&self) -> String {
        String::from(format!("On {} scored anger: {}, sadness: {}, fear: {}, joy: {}, surprise: {}, love: {}", 
                             &self.timestamp.date().to_string(), &self.anger, 
                             &self.sadness, &self.fear,
                             &self.joy, &self.surprise,
                             &self.love))
    }
}


#[derive(Debug, Clone)]
pub enum SecondaryAndTertiaryEmotions {
    Affection,
    Lust,
    Longing,
    Cheerfulness,
    Zest,
    Contenment,
    Pride,
    Optimism,
    Enthrallment,
    Relief,
    Surprise,
    Irritability,
    Exasperation,
    Rage,
    Disgust,
    Envy,
    Torment,
    Suffering,
    Sadness,
    Dissapointment,
    Shame,
    Neglect,
    Sympathy,
    Horror,
    Nervousness,
    // here start tertiary
    Adoration,
    Fondness,
    Liking,
    Attraction,
    Caring,
    Tenderness,
    Compassion,
    Sentimentality,
    Desire, 
    Passion, 
    Inatuation,
    Amusement,
    Bliss,
    Gaiety,
    Glee,
    Jolliness,
    Joviality,
    Delight,
    Enjoyment,
    Gladness,
    Happiness,
    Jubiliation,
    Elation,
    Satisfaction, 
    Ecstasy,
    Euphoria,
    Enthusiasm,
    Zeal, 
    Excitement,
    Thrill,
    Exhallaration,
    Pleasure,
    Triumph,
    Eagerness,
    Hope,
    Rapture,
    Amazement,
    Astonishment,
    Aggravation,
    Agitation,
    Annoyance,
    Grouchy,
    Grumpy,
    Crosspatch,
    Frustration,
    Outrage,
    Fury,
    Wrath,
    Hostility,
    Ferocity,
    Bitterness,
    Hatred,
    Scorn,
    Spite,
    Vengefulness,
    Dislike,
    Resentment,
    Jealousy,
    Agony, 
    Anguish,
    Hurt,
    Depression, 
    Despair,
    Gloom, 
    Glumness,
    Unhappiness,
    Grief,
    Sorrow,
    Woe,
    Misery,
    Melancholy,
    Dismay, 
    Displeasure,
    Guilt,
    Regret,
    Remorse,
    Alienation,
    Defeatism,
    Dejection,
    Embarrassment,
    Homesickness,
    Humiliation,
    Insecurity,
    Insult,
    Isolation,
    Loneliness,
    Rejection,
    Pity,
    Alarm,
    Shock,
    Fright,
    Terror,
    Panic,
    Hysteria,
    Mortification,
    Anxiety,
    Suspense,
    Uneasiness,
    Apprehension,
    Worry,
    Distress,
    Dread
}


#[test]
fn to_string() {
    let today = Utc::now().date().to_string();
    let secondary: HashMap<SecondaryAndTertiaryEmotions, i16> = HashMap::new();
    let emotions = Emotions{timestamp: Utc::now(), anger: 0, sadness: 2, fear: 2, 
        joy: 4, surprise: 0, love: 2, secondary, comment: None};

    assert_eq!(emotions.to_string(), format!("On {} scored anger: 0, sadness: 2, fear: 2, joy: 4, surprise: 0, love: 2", today));
}
