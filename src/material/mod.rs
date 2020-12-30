mod manager;
mod texture;
mod util;

// For external use
pub use manager::TextureManager;
pub use texture::Texture;

// For internal use
use super::{AssetHolder, TextureHandle};
use texture::{RawTextureData, SubTexture};

// Internal Types
type InternalHandle = usize;