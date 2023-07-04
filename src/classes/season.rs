use core::fmt;

// Crazy bad enum
#[derive(Clone, Copy, Debug)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

// Implement a to_string for Season
// But because enum is a self made type it doesn't have a to string
// So we use a match
impl fmt::Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Season::Autumn => write!(f, "Autumn"),
            Season::Spring => write!(f, "Spring"),
            Season::Summer => write!(f, "Summer"),
            Season::Winter => write!(f, "Winter"),
        }
    }
}

// Implement a function iterator
// You normally cannot iterate over an enum
// Kinda redundant because the values() function returns an array on which you CAN iterate
impl Season {
    pub fn iterator() -> impl Iterator<Item = Season> {
        [
            Season::Spring,
            Season::Summer,
            Season::Autumn,
            Season::Winter,
        ]
        .iter()
        .copied()
    }
}

// Implement a way to get all values of an enumerator

impl Season {
    pub fn values() -> [Season; 4] {
        [
            Season::Spring,
            Season::Summer,
            Season::Autumn,
            Season::Winter,
        ]
    }
}
