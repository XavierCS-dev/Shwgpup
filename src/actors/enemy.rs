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

pub struct Enemy {
    entity: Entity,
    collision: Collision2D,
    alive: bool,
    direction: f32,
    velocity: f64,
}

impl Enemy {
    pub fn new(filepath: &str,
        x: u32,
        y: u32,
        velocity: f64,
        rotation: f32,
        scale: f32,
        surface: &Surface,
        config: &SurfaceConfiguration,
        adapter: &wgpu::Adapter,
        queue: &wgpu::Queue,
        device: &wgpu::Device,) -> Self {
        let entity = Entity::new(filepath, x, y, rotation, scale, surface, config, adapter, queue, device);
        let collision = Collision2D::new(entity.position, entity.sprite.diffuse_texture.texture.width() as f32, entity.sprite.diffuse_texture.texture.height() as f32, scale);
        Enemy {
            entity,
            collision,
            alive: true,
            direction: -1.0,
            velocity,
        }
    }

    pub fn update(&mut self, time_elapsed: &Duration, screen_height: f32, screen_width: f32) {
        if self.entity.position.y <= 0.0 {
            self.alive = false;
        }
        if self.entity.position.x >= screen_width || self.entity.position.x <= 0.0 {
            self.direction *= -1.0;
        }
        let x = self.entity.position.x + ((self.velocity / 1.5) * time_elapsed.as_secs_f64()) as f32 * self.direction;
        let y = self.entity.position.y - (self.velocity * time_elapsed.as_secs_f64()) as f32;
        self.entity.update(x as u32, y as u32, self.entity.rotation(), self.entity.scale());
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


impl Draw for Enemy {
    fn draw(&self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView) -> Result<(), wgpu::SurfaceError> {
            self.entity.render(device, encoder, view)
        }
}
