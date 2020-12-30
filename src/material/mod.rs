mod manager;
mod texture;
mod util;

// For external use
pub use manager::TextureManager;

// For internal use
use super::{AssetHolder, TextureHandle};
use texture::{RawTextureData, Texture, SubTexture};

// Internal Types
type InternalHandle = usize;