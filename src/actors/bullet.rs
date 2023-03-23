use crate::engine::collision_2d::Collision2D;
use crate::engine::entity::Entity;
use crate::engine::draw::Draw;
use wgpu::Surface;
use wgpu::Queue;
use wgpu::Device;
use wgpu::Adapter;
use wgpu::SurfaceConfiguration;
use std::time::Duration;
use cgmath::Vector2;

pub struct Bullet {
    entity: Entity,
    collision: Collision2D,
    alive: bool,
}

impl Bullet {
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
        let collision = Collision2D::new(entity.position, entity.sprite.diffuse_texture.texture.width() as f32, entity.sprite.diffuse_texture.texture.height() as f32, scale);
        Bullet {
            entity,
            collision,
            alive: true,
        }
    }

    pub fn update(&mut self, time_elapsed: &Duration, screen_height: f32) {
        if self.entity.position.y > screen_height {
            self.alive = false;
        }
        let y = self.entity.position.y + (4000.0 * time_elapsed.as_secs_f64()) as f32;
        self.entity.update(self.entity.position_x(), y as u32, self.entity.rotation(), self.entity.scale());
        self.collision.update(self.entity.position);
    }

    pub fn get_collision(&self) -> &Collision2D {
        &self.collision
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }

    pub fn alive(&self) -> bool {
        self.alive
    }
}

impl Draw for Bullet {
    fn draw(&self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView) -> Result<(), wgpu::SurfaceError> {
            self.entity.render(device, encoder, view)
        }
}
