#[derive(Clone)]
pub enum Priority {
    LOW,
    MODERATE,
    HIGH,
}

impl Priority {
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::LOW => 2,
            Self::MODERATE => 1,
            Self::HIGH => 0,
        }
    }

    pub fn is_high(&self) -> bool {
        self.to_u8() == Self::HIGH.to_u8()
    }

    pub fn color<S: AsRef<str>>(&self, s: S) -> String {
        match self {
            Self::LOW => format!("\x1b[38;5;240;1m{}\x1b[0m", s.as_ref()),
            Self::MODERATE => format!("\x1b[38;5;31;1m{}\x1b[0m", s.as_ref()),
            Self::HIGH => format!("\x1b[38;5;203;1m{}\x1b[0m", s.as_ref()),
        }
    }

    pub fn background<S: AsRef<str>>(&self, s: S) -> String {
        match self {
            Self::LOW => format!("\x1b[48;5;240;38;5;248;1m {} \x1b[0m", s.as_ref()),
            Self::MODERATE => format!("\x1b[48;5;31;38;5;232;1m {} \x1b[0m", s.as_ref()),
            Self::HIGH => format!("\x1b[48;5;203;38;5;232;1m {} \x1b[0m", s.as_ref()),
        }
    }
}
