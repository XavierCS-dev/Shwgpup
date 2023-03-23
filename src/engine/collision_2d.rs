use cgmath::Vector2;

// Rectangular bonding box represented by an origin, width, and height
pub struct Collision2D {
    origin: Vector2<f32>,
    width: f32,
    height: f32,
}

impl Collision2D {
    pub fn new(origin: Vector2<f32>, width: f32, height: f32, scale: f32) -> Self {
        let width = width * scale;
        let height = height * scale;
        Collision2D {
            origin,
            width,
            height,
        }
    }

    // LIkely only origin will change, scale moved to different method
    pub fn update(&mut self, origin: Vector2<f32>) {
        self.origin = origin;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.width *= scale;
        self.height *= scale;
    }

    // Returns true if the ractangles overlap
    pub fn check_collision(&self, other: &Collision2D) -> bool {
        let dist_ax = self.width / 2.0;
        let dist_ay = self.height / 2.0;
        let dist_bx = other.width / 2.0;
        let dist_by = other.height / 2.0;
        (self.origin.x + dist_ax) > (other.origin.x - dist_bx) &&
        (self.origin.x - dist_ax) < (other.origin.x + dist_bx) &&
        (self.origin.y + dist_ay) > (other.origin.y - dist_by) &&
        (self.origin.y - dist_ay) < (other.origin.y + dist_by)
    }

}
