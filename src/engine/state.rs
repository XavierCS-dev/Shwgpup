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

pub struct State {
    pub surface: wgpu::Surface,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub entities: Vec<Entity>,
    pub entity_groups: Vec<EntityGroup>,
    pub window: Window,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

static mut rotation: f32 = 0.0;

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: Window) -> Self {
        // locking the window size to prevent scaling issues with sprites
        // can fix this if window is resized, but this is costly,
        // game will be too large on high res screens, too small on low res
        // screens, will need to work out a nice aspect ratio, then apply
        // scaling where appropriate.
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
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        let mut entities: Vec<Entity> = Vec::new();
        for i in 0..0 {
            entities.push(Entity::new(
                "assets/spoon.png",
                i,
                i,
                0.0,
                0.35,
                &surface,
                &config,
                &adapter,
                &queue,
                &device,
            ));
        }

        let mut entity_groups: Vec<EntityGroup> = Vec::new();
        let mut group = EntityGroup::new("assets/spoon.png",&surface, &config, &adapter, &queue, &device);
        for i in 0..1 {
            group.add_instance(i, i, i, 0.0, 0.35, &device).unwrap();
        }
        entity_groups.push(group);


        // ...
        Self {
            window,
            surface,
            config,
            size,
            entities,
            entity_groups,
            device,
            queue,
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
        false
    }

    pub fn update(&mut self) {
        unsafe { rotation += 0.2 };
        for entity in &mut self.entities {
            entity.update(
                entity.position_x(),
                entity.position_y(),
                unsafe { rotation },
                entity.scale(),
            );
        }
        for group in &mut self.entity_groups {
            for i in 0..group.count() {
                let mut instance = group.get_instance(i as u32).unwrap();
                instance.update(instance.position_x(), instance.position_y(), unsafe {rotation}, instance.scale());
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
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }
        for entity in &self.entities {
            entity.render(&self.device, &mut encoder, &view).unwrap();
        }
        for group in &self.entity_groups {
            group.render(&self.device, &mut encoder, &view).unwrap();
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
