mod manager;
mod uniforms;

// for internal use
use super::LineInstance; 
use uniforms::{CommonUniform, SharedUniform};

// public for external use
pub use manager::{PipelineManager,MAX_INSTANCES, MAX_QUADS_PER_DRAW};