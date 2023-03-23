use crate::engine::collision_2d::Collision2D;
use crate::engine::entity::Entity;
use crate::engine::draw::Draw;
use wgpu::Surface;
use wgpu::Queue;
use wgpu::Device;
use wgpu::Adapter;
use wgpu::SurfaceConfiguration;
use std::time::Duration;

pub struct Player {
    entity: Entity,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Player {
    pub fn new(filepath: &str,
        x: u32,
        y: u32,
        rotation: f32,
        scale: f32,
        surface: &Surface,
        config: &SurfaceConfiguration,
        adapter: &wgpu::Adapter,
        queue: &wgpu::Queue,
        device: &wgpu::Device,) -> Self {
        let entity = Entity::new(filepath, x, y, rotation, scale, surface, config, adapter, queue, device);
        Player {
            entity,
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
    pub fn update(&mut self, time_elapsed: &Duration, screen_width: u32, screen_height: u32) {
        let elapsed = time_elapsed.as_nanos() as f64;
        let mut x = self.entity.position_x() as i32;
        let mut y = self.entity.position_y() as i32;
        let screen_height = screen_height as i32;
        let screen_width = screen_width as i32;
        println!("elapsed: {}", elapsed);
        if self.left {
            x = self.entity.position_x() as i32 - ((0.090 * elapsed) / 100000.0) as i32;
            if x <= 0 {
                x = 0;
            }
        }
        if self.right {
            x = self.entity.position_x() as i32 + ((0.090 * elapsed) / 100000.0) as i32;
            if x >= screen_width {
                x = screen_width;
            }
        }
        if self.up {
            y = self.entity.position_y() as i32 + ((0.125 * elapsed) / 100000.0) as i32;
            if y >= screen_height {
                y = screen_height;
            }
        }
        if self.down {
            y = self.entity.position_y() as i32 - ((0.125 * elapsed) / 100000.0) as i32;
            if y <= 0 {
                y = 0;
            }
        }
        self.entity.update(x as u32, y as u32, self.entity.rotation(), self.entity.scale());
    }
}

impl Draw for Player {
    fn draw(&self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView) -> Result<(), wgpu::SurfaceError> {
            self.entity.render(device, encoder, view)
        }
}
