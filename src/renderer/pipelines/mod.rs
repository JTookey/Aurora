mod manager;
mod pipeline_2d;
mod pipeline_lines;
mod pipeline_text;
mod uniforms;
mod util;

// for internal use
use super::{LineInstance, TwoDInstance, Texture, Section}; 
use pipeline_2d::TwoDPipeline;
use pipeline_lines::LinesPipeline;
use pipeline_text::TextPipeline;
use uniforms::{CommonUniform, SharedUniform};

// public for external use
pub use manager::{PipelineManager,MAX_INSTANCES};