mod commands;
mod instances;
mod pipelines;
mod renderer;

// public for external use
pub use renderer::RendererInstance;

// for internal use
use commands::CommandManager;

pub trait Renderer {}