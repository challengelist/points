use crate::calculator::{
    PointCalculator
};

pub struct StandardPointCalculator {
    pub position: i32,
    pub victor_count: i32,
    pub list_size: i32
}

impl PointCalculator for StandardPointCalculator {
    // 250 * EXP(LN(250 / 5) / (1 - list_size) * (demon - 1))
    // 250f64 * f64::exp(f64::ln(250f64 / 15f64) / (1f64 - 75f64) * (f64::from(self.base.position) - 1f64));
    fn calculate(&self) -> f64 {
        let top_value: f64 = 250.0;
        let low_value: f64 = 5.0;
        
        250.0f64 * f64::exp(f64::ln(top_value / low_value) / (1.0f64 - self.list_size as f64) * (f64::from(self.position) - 1.0f64))
    }
}