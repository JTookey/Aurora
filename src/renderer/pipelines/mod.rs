mod manager;
mod pipeline_lines;
mod uniforms;
mod util;

// for internal use
use super::LineInstance; 
use pipeline_lines::LinesPipeline;
use uniforms::{CommonUniform, SharedUniform};

// public for external use
pub use manager::{PipelineManager,MAX_INSTANCES, MAX_QUADS_PER_DRAW};