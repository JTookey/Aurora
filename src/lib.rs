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
pub type Section<'s> = wgpu_glyph::Section<'s>;
pub type Text<'t> = wgpu_glyph::Text<'t>;
pub type Point2 = cgmath::Point2<f32>;
pub type Point3 = cgmath::Point3<f32>;
pub type Vector2 = cgmath::Vector2<f32>;
pub type Vector3 = cgmath::Vector3<f32>;
pub type WindowSize = winit::dpi::PhysicalSize<u32>;
pub type WindowEvent<'a> = winit::event::WindowEvent<'a>;
pub type KeyCode = winit::event::VirtualKeyCode;
pub type KeyState = winit::event::ElementState;
pub type MouseButton = winit::event::MouseButton;
pub type LineBreaker = dyn wgpu_glyph::LineBreaker;
pub type Layout = wgpu_glyph::Layout<LineBreaker>;
pub type HorizontalAlign = wgpu_glyph::HorizontalAlign;
pub type VerticalAlign = wgpu_glyph::VerticalAlign;

// Handles
pub type GeometryHandle = usize;
pub type RendererHandle = usize;
pub type TextureHandle = usize;

// Enums
#[derive(Debug)]
enum AssetHolder<T,P> {
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
pub use renderer::{RenderCommand, RendererInstance, LineDescription, TwoDDescription, TwoDTypes};

// For internal use
use renderer::{CommandManager, CommandProcessor, SectionManager};
use material::Texture;