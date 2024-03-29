use crate::engine::entity::Entity;
use crate::engine::entity::EntityRaw;
use crate::engine::sprite::Sprite;
use crate::engine::texture;
use crate::engine::vertex::Vertex;
use cgmath::prelude::*;
use wgpu::util::DeviceExt;
use winit::event::WindowEvent;
use winit::window::Window;
use crate::engine::entity_group::EntityGroup;
use crate::actors::player::Player;
use crate::actors::enemy::Enemy;
use crate::actors::bullet::Bullet;
use crate::engine::draw::Draw;
use winit::event::KeyboardInput;
use winit::event::ElementState;
use winit::event::VirtualKeyCode;
use std::time::Instant;
use std::time::Duration;
use rand::Rng;

pub struct State {
    pub surface: wgpu::Surface,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub bullets: Vec<Bullet>,
    pub window: Window,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub instant: Instant,
    pub duration: Duration,
    pub adapter: wgpu::Adapter,
}

static mut rotation: f32 = 0.0;

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: Window) -> Self {
        window.set_inner_size(winit::dpi::PhysicalSize {
            width: 562,
            height: 1021,
        });
        window.set_resizable(false);
        let size = window.inner_size();
        // println!("size: {:?}", window.inner_size());

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        let player = Player::new("assets/player.png", 200, 200, 0.0, 4.0, &surface, &config, &adapter, &queue, &device);
        let enemies: Vec<Enemy> = Vec::new();
        let bullets: Vec<Bullet> = Vec::new();


        // ...
        let instant = Instant::now();
        let duration = instant.elapsed();
        Self {
            window,
            surface,
            config,
            size,
            player,
            enemies,
            bullets,
            device,
            queue,
            instant,
            duration,
            adapter,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        /*
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
        */
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                },
                ..
            } => {
                let pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::Up => {
                        self.player.up = pressed;
                        true
                    }
                    VirtualKeyCode::Down => {
                        self.player.down = pressed;
                        true
                    }
                    VirtualKeyCode::Left => {
                        self.player.left = pressed;
                        true
                    }
                    VirtualKeyCode::Right => {
                        self.player.right = pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }

    }

    pub fn update(&mut self) {
        self.duration = self.instant.elapsed();
        self.instant = Instant::now();
        self.player.update(&self.duration, self.window.inner_size().width, self.window.inner_size().height);
        self.enemies.retain_mut(|x| x.alive());
        self.bullets.retain_mut(|x| x.alive());
        let inner_size = self.window.inner_size();
        for bullet in &mut self.bullets {
            bullet.update(&self.duration, inner_size.height as f32);
        }
        if self.bullets.len() < 1 {
            self.bullets.push(Bullet::new("assets/bullet.png", self.player.x(), self.player.y(), 0.0, 2.0, &self.surface, &self.config, &self.adapter, &self.queue, &self.device));
        }
        for enemy in &mut self.enemies {
            enemy.update(&self.duration, inner_size.height as f32, inner_size.width as f32);
        }
        if self.enemies.len() < 15 {
            let mut rng = rand::thread_rng();
            let x: u32 = rng.gen_range(0..self.window.inner_size().width);
            let y: u32 = rng.gen_range(self.window.inner_size().height..self.window.inner_size().height * 2);
            let velocity: f64 = rng.gen_range(300.0..700.0);
            self.enemies.push(Enemy::new("assets/enemy.png", x, y, velocity, 0.0, 2.0, &self.surface, &self.config, &self.adapter, &self.queue, &self.device));
        }
        for enemy in &mut self.enemies {
            for bullet in &mut self.bullets {
                if enemy.get_collision().check_collision(bullet.get_collision()) {
                    bullet.kill();
                    enemy.kill();
                    break;
                }
            }
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }
        for bullet in &mut self.bullets {
            bullet.draw(&self.device, &mut encoder, &view).unwrap();
        }
        for enemy in &mut self.enemies {
            enemy.draw(&self.device, &mut encoder, &view).unwrap();
        }
        self.player.draw(&self.device, &mut encoder, &view).unwrap();
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
