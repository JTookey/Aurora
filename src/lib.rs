mod baseapp;
mod geometry;
mod material;
mod renderer;
mod setup;
mod start;

// Re-exports
//pub use cgmath::Point2 as _cgmath_point2;

// Types
pub type Colour = wgpu::Color;
pub type Point2 = cgmath::Point2<f32>;
pub type Point3 = cgmath::Point3<f32>;
pub type Vector2 = cgmath::Vector2<f32>;
pub type Vector3 = cgmath::Vector3<f32>;
pub type WindowEvent<'a> = winit::event::WindowEvent<'a>;

// Handles
pub type GeometryHandle = usize;
pub type RendererHandle = usize;
pub type TextureHandle = usize;

// Enums
#[derive(Debug)]
enum AssetHolder<T,P> {
    Unloaded(String),
    Unprepared(P),
    Loaded(T),
}

// Traits
pub use baseapp::BaseApp;
pub use renderer::Renderer;

// Funtions and Structure for setting up Aurora and running
pub use setup::*;
pub use start::run;

// Main Structures that need making externally available
pub use geometry::GeometryManager;
pub use material::TextureManager;
pub use renderer::{RenderCommand, RendererInstance, LineDescription};

// For internal use
use renderer::CommandManager;