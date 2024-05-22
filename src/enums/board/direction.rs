use strum_macros::Display;
use strum_macros::EnumIter;

#[derive(Debug, Display, Clone, Copy, EnumIter)]
pub enum DIRECTION {
    UpDown = 8,
    LeftRight = 1,
}

impl DIRECTION {
    pub fn bits(&self) -> u64 {
        (*self as i8).abs() as u64
    }
}
