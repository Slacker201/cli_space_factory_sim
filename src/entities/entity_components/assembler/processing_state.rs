use std::ops::{AddAssign, SubAssign};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
/// The processing state enum.
/// This is used to track various factory processes
pub enum ProcessingState {
    /// Idle means the factory is idle, and is not producing anything
    Idle,
    /// Processing means the factory is processing.
    /// The value in processing is the current processing tick
    Processing(u32),
}

impl AddAssign<u32> for ProcessingState {
    /// This adds a number to the enum. If the enum is idle, it becomes Processing(x) where x is the number added. If it is already Processing(n) it becomes Processing(n+x)
    fn add_assign(&mut self, rhs: u32) {
        match self {
            // If currently Idle, adding n makes a new Processing(n) enum
            ProcessingState::Idle => {
                *self = ProcessingState::Processing(rhs);
            }
            ProcessingState::Processing(x) => {
                *x += rhs;
            }
        }
    }
}
impl SubAssign<u32> for ProcessingState {
    /// This adds a number to the enum. If the enum is idle, it becomes Processing(x) where x is the number added. If it is already Processing(n) it becomes Processing(n+x)
    fn sub_assign(&mut self, rhs: u32) {
        match self {
            // If currently Idle, adding n makes a new Processing(n) enum
            ProcessingState::Idle => {
                *self = ProcessingState::Processing(0);
            }
            ProcessingState::Processing(x) => {
                *x -= rhs;
            }
        }
    }
}
