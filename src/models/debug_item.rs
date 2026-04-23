#[derive(Debug, Clone)]
pub struct DebugItem {
    pub x: f64,
    pub y: f64,
    pub vy: f64,
    pub symbol: char,
}

impl DebugItem {
    pub fn new(x: f64, y: f64, symbol: char) -> Self {
        Self {
            x,
            y,
            vy: 0.0,
            symbol,
        }
    }
}