use std::fmt::{Display, Formatter};

pub enum Border {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

impl Border {
    pub const fn fmt(&self) -> &str {
        match self {
            Self::TopLeft => "╭",
            Self::Top | Self::Bottom => "─",
            Self::TopRight => "╮",
            Self::Right | Self::Left => "│",
            Self::BottomRight => "╯",
            Self::BottomLeft => "╰",
        }
    }
}

impl AsRef<str> for Border {
    fn as_ref(&self) -> &str {
        self.fmt()
    }
}

impl Display for Border {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt())
    }
}

pub enum Edge {
    Top,
    Bottom,
}

impl Edge {
    pub fn fmt(self, width: usize) -> String {
        match self {
            Self::Top => format!(
                "{}{: >width$}{}",
                Border::TopLeft,
                Border::Top.fmt().repeat(width),
                Border::TopRight
            ),
            Self::Bottom => format!(
                "{}{: >width$}{}",
                Border::BottomLeft,
                Border::Bottom.fmt().repeat(width),
                Border::BottomRight
            ),
        }
    }
}

pub enum Separator {
    Single,
    Blank,
}

impl Separator {
    pub fn fmt(self, width: usize) -> String {
        match self {
            Self::Single => format!("├{:─>width$}┤", ""),
            Self::Blank => format!(
                "{}{: >width$}{}",
                Border::Left.fmt(),
                "",
                Border::Right.fmt()
            ),
        }
    }
}
