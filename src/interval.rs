pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    /// Creates a new interval with the specified minimum and maximum.
    /// The minimum must be less than the maximum.
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Checks whether a given value is between the minimum and maximum values
    /// of the interval.
    pub fn contains(&self, x: f64) -> bool {
        x >= self.min && x <= self.max
    }
}

/// An interval that contains nothing.
pub static EMPTY: Interval = Interval::new(f64::INFINITY, f64::NEG_INFINITY);

/// An interval that contains everything.
pub static UNIVERSE: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);
