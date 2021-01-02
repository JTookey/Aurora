use crate::{
    GeometryManager, 
    TextureManager,
    Renderer,
};

use winit::event::WindowEvent;

pub trait BaseApp<'a>: 'static + Sized {
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
        geometry_manager: &mut GeometryManager,
        texture_manger: &mut TextureManager,
    ) -> Self;
    fn handle_input(&mut self, event: WindowEvent);
    fn update(&mut self, delta_t: f32);
    fn resize(&mut self);
    fn draw<R: Renderer<'a>>(&mut self, renderer: &mut R);
}