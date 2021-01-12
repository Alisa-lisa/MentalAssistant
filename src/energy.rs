/// Energy tracking form
/// ---
///
/// This is supposed to assess physical energy level to start 
/// tying psychological traits to physical
///
/// suggested scale 0..100 with battery (0..100%) analogy 
use crhono::{DateTime, Utc};


pub struct Energy {
    pub timestamp: DateTime<Utc>,
    pub level: i16, 
    pub coment: Option<String>
}


