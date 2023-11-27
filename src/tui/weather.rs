use anyhow::anyhow;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug)]
pub enum WindDirection {
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
}

impl WindDirection {
    pub fn get_direction(wind_direction: u16) -> anyhow::Result<Self> {
        let direction = match wind_direction % 360 {
            (337..=360) | (0..=22) => Self::North,
            (22..=67) => Self::NorthEast,
            (67..=112) => Self::East,
            (112..=157) => Self::SouthEast,
            (157..=202) => Self::South,
            (202..=247) => Self::SouthWest,
            (247..=292) => Self::West,
            (292..=337) => Self::NorthWest,
            _ => {
                return Err(anyhow!(
                    "Couldn't parse wind direction (this should never happen???)"
                ))
            }
        };

        Ok(direction)
    }

    pub const fn get_icon(&self) -> char {
        match *self {
            Self::North => '↓',
            Self::NorthEast => '↙',
            Self::East => '←',
            Self::SouthEast => '↖',
            Self::South => '↑',
            Self::SouthWest => '↗',
            Self::West => '→',
            Self::NorthWest => '↘',
        }
    }
}

impl Display for WindDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WindDirection::NorthWest => write!(f, "NW"),
            WindDirection::North => write!(f, "N"),
            WindDirection::NorthEast => write!(f, "NE"),
            WindDirection::East => write!(f, "E"),
            WindDirection::SouthEast => write!(f, "SW"),
            WindDirection::South => write!(f, "S"),
            WindDirection::SouthWest => write!(f, "SW"),
            WindDirection::West => write!(f, "W"),
        }
    }
}
