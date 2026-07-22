// z ordering
// higher is in front
#[derive(Debug)]
pub enum RadarZOrdering {
    Background = -1,
    Planet = 0,
    Ships = 1,
    Clouds = 2,
    Camera = 100, // max
}

impl RadarZOrdering {
    /// Returns "0.z", z being the interger value of this [`RadarZOrdering`].
    pub fn z_order(self) -> f32 {
        let f = self as i32 as f32;
        0.01 * f
    }
}
