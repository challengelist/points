use crate::calculator::{
    PointCalculator
};

pub struct StandardPointCalculator {
    pub position: i32,
    pub victor_count: i32,
    pub list_size: i32
}

impl PointCalculator for StandardPointCalculator {
    fn calculate(&self) -> f64 {
        0.0
    }
}