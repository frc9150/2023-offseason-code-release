#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
#[repr(transparent)]
pub struct CANId(pub i32);
