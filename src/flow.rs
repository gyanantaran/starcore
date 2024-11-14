pub enum Flow {
    Forward,
    Pause,
    //Rewind,
}

impl Flow {
    pub fn update(&mut self) {
        *self = match self {
            Self::Forward => Self::Pause,
            Self::Pause => Self::Forward,
        }
    }

    pub fn return_time_factor(self: &Self) -> i8 {
        match self {
            Self::Forward => 1,
            Self::Pause => 0,
        }
    }
}

impl Default for Flow {
    fn default() -> Self {
        Self::Pause
    }
}
