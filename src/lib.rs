mod baseapp;
mod geometry;
mod material;
mod renderer;
mod setup;
mod start;

// Traits
pub use baseapp::BaseApp;
pub use renderer::Renderer;

// Funtions and Structure for setting up Aurora and running
pub use setup::*;
pub use start::run;

// Main Structures that need making externally available
pub use geometry::GeometryManager;
pub use material::TextureManager;
pub use renderer::RendererInstance;

// Reexport Window Event
pub use winit::event::WindowEvent;