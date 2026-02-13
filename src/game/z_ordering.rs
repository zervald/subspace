// z ordering
// higher is in front
#[derive(Debug)]
pub enum RadarZOrdering {
    Background = -1,
    Planet = 0,
    Ships = 1,
    Clouds = 2,
}

impl RadarZOrdering {
    pub fn as_f32(&self) -> f32 {
        self as *const _ as i32 as f32
    }
}
