// z ordering
// higher is in front
#[derive(Debug)]
pub enum RadarOrdering {
    ZBackground = -1,
    ZPlanet = 0,
    ZShips = 1,
    ZClouds = 2,
}

impl RadarOrdering {
    pub fn as_f32(self) -> f32 {
        self as i32 as f32
    }
}
