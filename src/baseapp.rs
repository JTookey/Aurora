use crate::{
    GeometryManager, 
    TextureManager,
    Renderer,
    WindowSize,
};

use winit::event::WindowEvent;

pub trait BaseApp: 'static + Sized {
    fn optional_features() -> wgpu::Features {
        wgpu::Features::empty()
    }
    fn required_features() -> wgpu::Features {
        wgpu::Features::empty()
    }
    fn required_limits() -> wgpu::Limits {
        wgpu::Limits::default()
    }
    fn init(
        window_size: WindowSize,
        geometry_manager: &mut GeometryManager,
        texture_manger: &mut TextureManager,
    ) -> Self;
    fn handle_input(&mut self, event: WindowEvent);
    fn update(&mut self, delta_t: f32);
    fn resize(&mut self, size: WindowSize);
    fn draw<'draw, R: Renderer<'draw>>(&'draw mut self, renderer: R);
}