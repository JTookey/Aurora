mod manager;
mod uniforms;

// public for external use
pub use manager::PipelineManager;

// for internal use
use uniforms::{CommonUniform, SharedUniform};