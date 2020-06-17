#[derive(Debug, Clone)]
pub struct TravellingSalesmanCity {
    name: String,
    x: f32,
    y: f32,
}

impl TravellingSalesmanCity {
    pub fn new(name: String, x: f32, y: f32) -> Self {
        Self { name, x, y }
    }

    pub fn distance_to(&self, city: &TravellingSalesmanCity) -> f32 {
        let (dx, dy) = (self.x - city.x, self.y - city.y);
        (dx*dx + dy*dy).sqrt()
    }
}