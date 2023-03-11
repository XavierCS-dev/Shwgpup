use crate::texture;
use crate::vertex::Vertex;

// contains texture and texture bind group used for drawing
// vertices are just a rectangle to fit an image
// indicies form the two triangles to actually be drawn by the gpu.
pub struct Sprite {
    pub diffuse_texture: texture::Texture,
    pub diffuse_bind_group: wgpu::BindGroup,
    pub vertices: [Vertex; 4],
    pub indices: [u16; 6],
}

impl Sprite {
    pub fn new(
        filepath: &str,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
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
        let vertices = Sprite::create_vetices(image_file.width(), image_file.height())
        let indices = [0, 1, 2, 0, 2, 3];
        Sprite {
            diffuse_texture,
            diffuse_bind_group,
            vertices,
            indices,
        }
    }

    fn create_vetices(width: u32, height: u32, window_width: u32, window_height: u32) -> [Vertex; 4] {
        // normalise pixel dimensions of images to maintain aspect ratio and fit on the screen
        // which has range 0-1.
        // look to scale normalised size in the future..for now I will stick with a fixed resolution.
        [
            Vertex {
                position: [],
                tex_coords: [],
            },
            Vertex {
                position: [],
                tex_coords: [],
            },
            Vertex {
                position: [],
                tex_coords: [],
            },
            Vertex {
                position: [],
                tex_coords: [],
            },

        ]

    }

    fn normalise(given: f32, max: f32, min: f32) -> f32 {
        (given - min) / (max - min)
    }

    pub fn diffuse_bind_group(&self) -> &wgpu::BindGroup {
        &self.diffuse_bind_group
    }
}
