use crate::engine::render_init::RenderInit;
use crate::engine::sprite::Sprite;
use crate::engine::transformation::Transformation;
use crate::engine::vertex::Vertex;
use cgmath::prelude::*;
use cgmath::Basis2;
use cgmath::Vector2;
use wgpu::util::DeviceExt;
use wgpu::Queue;
use wgpu::Surface;
use wgpu::SurfaceConfiguration;
use wgpu::SurfaceTexture;
use wgpu::TextureView;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EntityRaw {
    pub position: [f32; 2],
    origin: [f32; 2],
    rotation: [[f32; 2]; 2],
    scale: [[f32; 2]; 2],
}

// contain sprite, This struct is for rare entities, ie not sharing a sprite.
// a shared entity with support for instancing will be created in that case.
// move position into transformation struct
pub struct Entity {
    sprite: Sprite,
    position: Vector2<f32>,
    transformation: Transformation,
    render_pipeline: wgpu::RenderPipeline,
    rotation_deg: f32,
    ent_scale: f32,
}

impl Entity {
    pub fn new(
        filepath: &str,
        x: u32,
        y: u32,
        rotation: f32,
        scale: f32,
        surface: &Surface,
        config: &SurfaceConfiguration,
        adapter: &wgpu::Adapter,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
    ) -> Entity {
        let rotation_deg = rotation;
        let x = x as f32;
        let y = y as f32;
        let ent_scale = scale;
        let position = Vector2 { x, y };
        let transformation = Transformation::new(rotation, scale);
        let render_init = pollster::block_on(RenderInit::new(config, device));
        let render_pipeline = render_init.render_pipeline;
        let texture_bind_group_layout = render_init.texture_bind_group_layout;
        let sprite = Sprite::new(filepath, &texture_bind_group_layout, &device, &queue);
        Entity {
            sprite,
            position,
            transformation,
            render_pipeline,
            rotation_deg,
            ent_scale,
        }
    }

    // update function to be called for entities, more complicated structures such as players or enemies will call this
    // on their entities to update their position, I will also possibly implement scaling.
    pub fn update(&mut self, x: u32, y: u32, rotation: f32, scale: f32) {
        self.rotation_deg = rotation;
        self.ent_scale = scale;
        self.position.x = x as f32;
        self.position.y = y as f32;
        self.transformation.update(rotation, scale);
    }

    // needed for sending to the shaders (rotation and position)
    pub fn to_raw(&self) -> EntityRaw {
        EntityRaw {
            position: self.position.into(),
            origin: self.sprite.origin.into(),
            // convert degrees to radians
            rotation: self.transformation.rotation(),
            scale: self.transformation.scale(),
        }
    }

    pub fn render(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
    ) -> Result<(), wgpu::SurfaceError> {
        let entity_data = vec![self.to_raw()];
        let entity_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&entity_data),
            usage: wgpu::BufferUsages::VERTEX,
        });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            // implement instancing...try and fixc the life time issue when using values in a render function
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.sprite.diffuse_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.sprite.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, entity_buffer.slice(..));
            render_pass.set_index_buffer(
                self.sprite.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            // println!("size: {:?}", self.window.inner_size());
            render_pass.draw_indexed(0..6, 0, 0..1);
        }

        Ok(())
    }

    pub fn position_x(&self) -> u32 {
        self.position.x as u32
    }
    pub fn position_y(&self) -> u32 {
        self.position.y as u32
    }
    pub fn rotation(&self) -> f32 {
        self.rotation_deg
    }
    pub fn scale(&self) -> f32 {
        self.ent_scale
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
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 10]>() as wgpu::BufferAddress,
                    shader_location: 10,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}
