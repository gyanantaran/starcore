pub enum TimeMode {
    Forward,
    Pause,
    Rewind,
}

impl TimeMode {
    pub fn update(self: &mut Self) {
        *self = match self {
            Self::Forward => Self::Pause,
            Self::Pause   => Self::Rewind,
            Self::Rewind => Self::Forward,
        }
    }
}