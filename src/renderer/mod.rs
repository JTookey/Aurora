mod command_executor;
mod command_manager;
mod command_processor;
mod instances;
mod pipelines;
mod renderer;
mod render_commands;
mod section_manager;

// public for external use
pub use renderer::RendererInstance;
pub use command_manager::CommandManager;
pub use command_processor::CommandProcessor;
pub use render_commands::{RenderCommand, LineDescription, TwoDDescription, TwoDTypes};
pub use section_manager::SectionManager;

// for internal use
use super::{TextureManager, Texture, Section};
use command_executor::CommandExecutor;
use command_manager::InternalCommands;
use instances::{LineInstance, TwoDInstance, ThreeDInstance};
use pipelines::{PipelineManager, MAX_INSTANCES, MAX_QUADS_PER_DRAW};

// Trait
pub trait Renderer<'sm> {
    fn add(&mut self, cmd: RenderCommand<'sm>);
}