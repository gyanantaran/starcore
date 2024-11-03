pub enum Flow {
    Forward,
    Pause,
    Rewind,
}

impl Flow {
    pub fn update(self: &mut Self) {
        *self = match self {
            Self::Forward => Self::Pause,
            Self::Pause => Self::Rewind,
            Self::Rewind => Self::Forward,
        }
    }

    pub fn return_time_factor(self: &Self) -> i8 {
        match self {
            Self::Forward => 1,
            Self::Pause => 0,
            Self::Rewind => -1,
        }
    }
}

impl Default for Flow {
    fn default() -> Self {
        Self::Pause
    }
}
