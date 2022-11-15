use crate::calculator::{
    PointCalculator
};

pub struct StandardPointCalculator {
    pub position: i32,
    pub victor_count: i32,
    pub list_size: i32
}

impl StandardPointCalculator {
    fn calculate_victor_bonus(&self, victor_max: f64) -> f64 {
        // Set constants for the calculation.
        let victor_multiplier = 2.5f64;

        // Calculates an exponential function based off the victor count.
        let victor_exponent = f64::exp((self.victor_count as f64 * victor_multiplier) / self.list_size as f64);

        // Return the victor bonus.
        victor_max / (victor_exponent * self.position as f64)
    }
}

impl PointCalculator for StandardPointCalculator {
    fn calculate(&self) -> f64 {
        if self.position > self.list_size {
            // Return nothing for legacy challenges.
            return 0.0f64
        }

        // Set constants for the calculation.
        let maximum: f64 = 250.0f64;
        let victor_maximum: f64 = 150.0f64;

        // Calculate a set divisor based off the size of the list.
        let position_divisor = self.list_size as f64 / f64::ln(maximum / 2.0f64);

        // Calculate the victor bonus.
        let victor_bonus = self.calculate_victor_bonus(victor_maximum);

        // Return the calculated points.
        (250.0f64 / (f64::powi((self.position as f64 - 1.0f64) / position_divisor, 2) + 1.0f64)) + victor_bonus
    }
}