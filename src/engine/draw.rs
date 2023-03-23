pub trait Draw {
    fn draw(&self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView) -> Result<(), wgpu::SurfaceError>;
}
