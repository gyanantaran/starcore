pub enum TimeMode {
    Forward,
    Pause,
    Rewind,
}

impl TimeMode {
    pub fn update(self: &mut Self) {
        *self = match self {
            Self::Forward => Self::Pause,
            Self::Pause => Self::Rewind,
            Self::Rewind => Self::Forward,
        }
    }

    pub fn return_time_factor(self: &Self) -> f64 {
        match self {
            Self::Forward => 1f64,
            Self::Pause => 0f64,
            Self::Rewind => -1.0f64,
        }
    }
}
