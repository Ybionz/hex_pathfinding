#[derive(Debug)]
pub struct FPoint {
    pub x: f64,
    pub y: f64,
}

impl FPoint {
    pub fn new(x: f64, y: f64) -> FPoint {
        FPoint { x, y }
    }
}
