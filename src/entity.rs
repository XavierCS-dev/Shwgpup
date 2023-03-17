use crate::sprite::Sprite;
use cgmath::prelude::*;
use cgmath::Basis2;
use cgmath::Vector2;

// wgsl compatible rotation and position matrix
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EntityRaw {
    pub position: [f32; 2],
    pub origin: [f32; 2],
    pub rotation: f32,
    pub scale: f32,
}

// contain sprite, This struct is for rare entities, ie not sharing a sprite.
// a shared entity with support for instancing will be created in that case.
pub struct Entity {
    pub sprite: Sprite,
    position: Vector2<f32>,
    rotation: f32,
    scale: f32,
}

impl Entity {
    pub fn new(sprite: Sprite, x: f32, y: f32, rotation: f32, scale: f32) -> Entity {
        let position = Vector2 { x, y };
        Entity {
            sprite,
            position,
            rotation,
            scale,
        }
    }

    // update function to be called for entities, more complicated structures such as players or enemies will call this
    // on their entities to update their position, I will also possibly implement scaling.
    pub fn update(&mut self, x: f32, y: f32, rotation: f32, scale: f32) {
        self.position.x = x;
        self.position.y = y;
        self.rotation = rotation;
        self.scale = scale;
    }

    // needed for sending to the shaders (rotation and position)
    pub fn to_raw(&self) -> EntityRaw {
        EntityRaw {
            position: self.position.into(),
            origin: self.sprite.origin.into(),
            // convert degrees to radians
            rotation: (self.rotation * 3.14159265) / 180.0,
            scale: self.scale,
        }
    }

    pub fn render<'a, 'b>(
        &self,
        render_pass: &mut wgpu::RenderPass<'b>,
        pipeline: &wgpu::RenderPipeline,
    ) {
        //
    }
}


impl EntityRaw {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<EntityRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32,
                },

            ],
        }
    }
}
