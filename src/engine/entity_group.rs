// Used for entities with the same texture that will be spawed many times.
use crate::engine::render_init::RenderInit;
use crate::engine::sprite::Sprite;
use crate::engine::transformation::Transformation;
use cgmath::Vector2;
use std::collections::HashMap;
use wgpu::util::DeviceExt;
use wgpu::Surface;
use wgpu::SurfaceConfiguration;


#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceRaw {
    pub position: [f32; 2],
    origin: [f32; 2],
    rotation: [[f32; 2]; 2],
    scale: [[f32; 2]; 2],
}

pub struct Instance {
    position: Vector2<f32>,
    transformation: Transformation,
    rotation_deg: f32,
    ent_scale: f32,
    origin: Vector2<f32>,
}

pub struct EntityGroup {
    sprite: Sprite,
    render_pipeline: wgpu::RenderPipeline,
    instances: HashMap<u32, Instance>,
    indices: Vec<u16>,
    index_buffer: wgpu::Buffer,
}

// Use default positions and scale as will be empty by default so it won't matter.
// then create add / remove function for entities.
// It may be better to use this for singular entities as well...
// when this is fully working..consider removing the other entity struct...
impl EntityGroup {
    pub fn new(
        filepath: &str,
        surface: &Surface,
        config: &SurfaceConfiguration,
        adapter: &wgpu::Adapter,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
    ) -> Self {
        let render_init = pollster::block_on(RenderInit::new(config, device));
        let render_pipeline = render_init.render_pipeline;
        let texture_bind_group_layout = render_init.texture_bind_group_layout;
        let sprite = Sprite::new(filepath, &texture_bind_group_layout, &device, &queue);
        let instances: HashMap<u32, Instance> = HashMap::new();
        let indices: Vec<u16> = Vec::new();
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        EntityGroup {
            sprite,
            render_pipeline,
            instances,
            indices,
            index_buffer,
        }
    }

    pub fn render(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
    ) -> Result<(), wgpu::SurfaceError> {
        if self.instances.len() == 0 {
            return Ok(());
        }
        let mut instance_data: Vec<InstanceRaw> = Vec::new();
        for instance in &self.instances {
            instance_data.push(instance.1.to_raw());
        }
        let entity_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
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
                self.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            // println!("size: {:?}", self.window.inner_size());
            render_pass.draw_indexed(0..(self.instances.len() * 6) as _, 0, 0..self.instances.len() as _);
        }

        Ok(())
    }

    pub fn remove_instance(&mut self, id: u32, device: &wgpu::Device) -> Result<(), &str> {
        if !self.instances.contains_key(&id) {
            return Err("Invalid id");
        }
        self.instances.remove(&id);
        for _ in 0..6 {
            self.indices.pop();
        }
        self.index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&self.indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        Ok(())
    }
    pub fn add_instance(
        &mut self,
        id: u32,
        x: u32,
        y: u32,
        rotation: f32,
        scale: f32,
        device: &wgpu::Device,
    ) -> Result<(), &str> {
        // insert will update the value at the key if it already exists... thus this is needed
        if self.instances.contains_key(&id) {
            return Err("ID already exists");
        }
        let instance = Instance::new(x, y, rotation, scale, self.sprite.origin);
        self.instances.insert(id, instance);
        self.indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);
        self.index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&self.indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        Ok(())
    }

    pub fn get_instance(&mut self, id: u32) -> Result<&mut Instance, &str> {
        self.instances.get_mut(&id).ok_or("Invalid id")
    }

    pub fn count(&self) -> usize {
        self.instances.len()
    }
}

impl Instance {
    fn new(x: u32, y: u32, rotation: f32, scale: f32, origin: Vector2<f32>) -> Self {
        let position = Vector2 {
            x: x as f32,
            y: y as f32,
        };
        let transformation = Transformation::new(rotation, scale);
        Instance {
            position,
            transformation,
            rotation_deg: rotation,
            ent_scale: scale,
            origin,
        }
    }

    pub fn update(&mut self, x: u32, y: u32, rotation: f32, scale: f32) {
        self.rotation_deg = rotation;
        self.ent_scale = scale;
        self.position.x = x as f32;
        self.position.y = y as f32;
        self.transformation.update(rotation, scale);
    }

    fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            position: self.position.into(),
            origin: self.origin.into(),
            // convert degrees to radians
            rotation: self.transformation.rotation(),
            scale: self.transformation.scale(),
        }
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
