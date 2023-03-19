#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Transformation {
    rotation: [[f32; 2]; 2],
    scale: [[f32; 2]; 2],
}

impl Transformation {
    pub fn new(rotation: f32, scale: f32) -> Transformation {
        let rotation = [
            [rotation.cos(), rotation.sin()],
            [-(rotation.sin()), rotation.cos()],
        ];
        let scale = [[scale, 0.0], [0.0, scale]];
        Transformation { rotation, scale }
    }

    pub fn rotation(&self) -> [[f32; 2]; 2] {
        self.rotation
    }

    pub fn scale(&self) -> [[f32; 2]; 2] {
        self.scale
    }
}
