use crate::texture;
use crate::vertex::Vertex;
use cgmath::Vector2;
use wgpu::util::DeviceExt;

// contains texture and texture bind group used for drawing
// vertices are just a rectangle to fit an image
// indicies form the two triangles to actually be drawn by the gpu.
pub struct Sprite {
    pub diffuse_texture: texture::Texture,
    pub diffuse_bind_group: wgpu::BindGroup,
    pub vertices: [Vertex; 4],
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub indices: [u16; 6],
    pub origin: Vector2<f32>,
}

impl Sprite {
    pub fn new(
        filepath: &str,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: &winit::dpi::PhysicalSize<u32>,
    ) -> Self {
        let image_file = image::open(filepath).unwrap();
        let diffuse_texture =
            texture::Texture::from_image(device, queue, &image_file, Some(filepath)).unwrap();
        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view), // CHANGED!
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler), // CHANGED!
                },
            ],
            label: Some("diffuse_bind_group"),
        });
        let (origin, vertices) = Sprite::create_vetices(
            image_file.width(),
            image_file.height(),
            size.width,
            size.height,
        );
        let indices = [0, 1, 2, 0, 2, 3];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        Sprite {
            diffuse_texture,
            diffuse_bind_group,
            vertices,
            vertex_buffer,
            index_buffer,
            indices,
            origin,
        }
    }

    fn create_vetices(
        width: u32,
        height: u32,
        window_width: u32,
        window_height: u32,
    ) -> (Vector2<f32>, [Vertex; 4]) {
        // normalise pixel dimensions of images to maintain aspect ratio and fit on the screen
        // which has range 0-1.
        // look to scale normalised size in the future..for now I will stick with a fixed resolution.
        let normal_width = Sprite::normalise(width as f32, window_width as f32, 0.0);
        let normal_height = Sprite::normalise(height as f32, window_height as f32, 0.0);
        (
            Vector2 {
                x: normal_width / 2.0,
                y: normal_height / 2.0,
            },
            [
                Vertex {
                    position: [normal_width, normal_height, 0.0],
                    tex_coords: [1.0, 0.0],
                },
                Vertex {
                    position: [0.0, normal_height, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [normal_width, 0.0, 0.0],
                    tex_coords: [1.0, 1.0],
                },
            ],
        )
    }

    fn normalise(given: f32, max: f32, min: f32) -> f32 {
        (given - min) / (max - min)
    }

    pub fn diffuse_bind_group(&self) -> &wgpu::BindGroup {
        &self.diffuse_bind_group
    }
}
