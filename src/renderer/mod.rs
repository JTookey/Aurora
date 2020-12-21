mod command_executor;
mod command_manager;
mod instances;
mod pipelines;
mod renderer;
mod render_commands;

// public for external use
pub use renderer::RendererInstance;
pub use render_commands::{RenderCommand, LineDescription};

// for internal use
use command_executor::CommandExecutor;
use command_manager::{CommandManager, InternalCommands};
use instances::{LineInstance, InstanceManager};
use pipelines::{PipelineManager, MAX_INSTANCES, MAX_QUADS_PER_DRAW};

// Trait
pub trait Renderer {
    fn add(&mut self, cmd: RenderCommand);
}