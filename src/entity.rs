use crate::sprite::Sprite;
use cgmath::Vector2;
use cgmath::Basis2;
use cgmath::prelude::*;



// wgsl compatible rotation and position matrix
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct EntityRaw {
    model: [[f32; 2]; 2],
}

// contain sprite, This struct is for rare entities, ie not sharing a sprite.
// a shared entity with support for instancing will be created in that case.
pub struct Entity {
    pub sprite: Sprite,
    pub position: Vector2<f32>,
    pub rotation: Basis2<f32>,
}


impl Entity {
    // update function to be called for entities, more complicated structures such as players or enemies will call this
    // on their entities to update their position, I will also possibly implement scaling.
    pub fn update(&mut self, position: Vector2<f32>, rotation: f32) {
        self.rotation =  Basis2::from_angle(cgmath::Deg(rotation).normalize());
        self.position = position;
    }

    // needed for sending to the shaders (rotation and position)
    pub fn to_raw() -> EntityRaw {
        todo!()
    }
}
