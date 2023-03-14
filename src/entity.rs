use crate::sprite::Sprite;
use cgmath::prelude::*;
use cgmath::Basis2;
use cgmath::Vector2;

// wgsl compatible rotation and position matrix
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EntityRaw {
    pub model: [[f32; 3]; 3],
}

// contain sprite, This struct is for rare entities, ie not sharing a sprite.
// a shared entity with support for instancing will be created in that case.
pub struct Entity {
    pub sprite: Sprite,
    position: Vector2<f32>,
    rotation: Basis2<f32>,
    raw: EntityRaw,
}

impl Entity {
    pub fn new(sprite: Sprite, x: f32, y: f32, rotation_deg: f32, scale: f32) -> Entity {
        let rotation = Basis2::from_angle(cgmath::Deg(rotation_deg).normalize());
        let position = rotation.rotate_vector(Vector2 { x, y });
        let init = cgmath::Matrix3::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        let raw = Entity::to_raw(
            init,
            cgmath::Matrix3::from_translation(position),
            cgmath::Matrix3::from_scale(scale),
        );
        Entity {
            sprite,
            position,
            rotation,
            raw,
        }
    }

    // update function to be called for entities, more complicated structures such as players or enemies will call this
    // on their entities to update their position, I will also possibly implement scaling.
    pub fn update(&mut self, x: f32, y: f32, rotation: f32, scale: f32) {
        self.position.y += y;
        self.position.x += x;
        self.rotation = Basis2::from_angle(cgmath::Deg(rotation).normalize());
        self.position = self.rotation.rotate_vector(self.position);
        let temp = cgmath::Matrix3 {
            x: self.raw.model[0].into(),
            y: self.raw.model[1].into(),
            z: self.raw.model[2].into(),
        };
        self.raw = Entity::to_raw(
            temp,
            cgmath::Matrix3::from_scale(scale),
            cgmath::Matrix3::from_translation(self.position),
        );
    }

    // needed for sending to the shaders (rotation and position)
    pub fn to_raw(
        original: cgmath::Matrix3<f32>,
        movement: cgmath::Matrix3<f32>,
        scale: cgmath::Matrix3<f32>,
    ) -> EntityRaw {
        EntityRaw {
            model: (original * movement * scale).into(),
        }
    }

    pub fn render<'a, 'b>(&self, render_pass: &mut wgpu::RenderPass<'b>, pipeline: &wgpu::RenderPipeline) {
        //
    }
}
