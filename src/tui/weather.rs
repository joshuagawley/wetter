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
        match wind_direction % 360 {
            wind_direction
                if (337..=360).contains(&wind_direction) || (0..22).contains(&wind_direction) =>
            {
                Ok(Self::North)
            }
            wind_direction if (22..67).contains(&wind_direction) => Ok(Self::NorthEast),
            wind_direction if (67..112).contains(&wind_direction) => Ok(Self::East),
            wind_direction if (112..157).contains(&wind_direction) => Ok(Self::SouthEast),
            wind_direction if (157..202).contains(&wind_direction) => Ok(Self::South),
            wind_direction if (202..247).contains(&wind_direction) => Ok(Self::SouthWest),
            wind_direction if (247..292).contains(&wind_direction) => Ok(Self::West),
            wind_direction if (292..337).contains(&wind_direction) => Ok(Self::NorthWest),
            _ => Err(anyhow!(
                "Couldn't parse wind direction (this should never happen???)"
            )),
        }
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
