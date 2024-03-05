#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn default() -> Self { Self { min: f64::INFINITY, max: f64::NEG_INFINITY } }
    pub fn new(min: f64, max: f64) -> Self { Self { min, max } }
    pub fn empty() -> Self { Self { min: f64::INFINITY, max: f64::NEG_INFINITY } }
    pub fn universe() -> Self { Self { min: f64::NEG_INFINITY, max: f64::INFINITY } }
    pub fn contains(&self, x: f64) -> bool { return self.min <= x && x <= self.max; }
    pub fn surrounds(&self, x: f64) -> bool { return self.min < x && x < self.max; }
}