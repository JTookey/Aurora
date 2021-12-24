
use super::{
    InternalHandle,
    util::create_gpu_texture,
};

// Unprepared Texture
pub struct RawTextureData {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

// Texture sub region
pub struct SubTexture {
    pub texture: InternalHandle,
    pub texture_position: [u32; 2],
    pub texture_size: [u32; 2],
}

// Handling and holding Textures in memory
pub struct Texture {
    pub texture_extent: wgpu::Extent3d,
    pub texture_buffer: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
    pub texture_sampler: wgpu::Sampler,
}

impl Texture {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let (texture_extent, texture_buffer, texture_sampler) = create_gpu_texture(device, width, height);
        let texture_view = texture_buffer.create_view(&wgpu::TextureViewDescriptor::default());
        
        Self {
                texture_extent,
                texture_buffer,
                texture_view,
                texture_sampler,
            }
    }

    // Function for creating a depth texture
    pub fn create_depth_texture(device: &wgpu::Device, sc_desc: &wgpu::SurfaceConfiguration) -> Self {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: sc_desc.width,
                height: sc_desc.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST,
            label: None,
        });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let depth_sampler = device.create_sampler(&wgpu::SamplerDescriptor { // 4.
            label: Some("Depth Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: Some(wgpu::CompareFunction::Always),
            anisotropy_clamp: None,
            border_color: None,
        });

        let depth_extent = wgpu::Extent3d {
            depth_or_array_layers: 1,
            height: sc_desc.height,
            width: sc_desc.width,
        };

        Self {
            texture_extent: depth_extent,
            texture_buffer: depth_texture,
            texture_view: depth_view,
            texture_sampler: depth_sampler, 
        }
    } 
    
    // Return the size of the texture
    pub fn get_size(&self) -> (u32, u32) {
        (self.texture_extent.width, self.texture_extent.height)
    }

    // Return the texture buffer
    pub fn get_texture_buffer(&self) -> &wgpu::Texture {
        &self.texture_buffer
    }

    // Return the Texture View
    pub fn get_view(&self) -> &wgpu::TextureView {
        &self.texture_view
    }

    // Return the Texture Sampler
    pub fn get_sampler(&self) -> &wgpu::Sampler {
        &self.texture_sampler
    }

    // Return the extent
    pub fn get_extent(&self) -> wgpu::Extent3d {
        self.texture_extent.clone()
    }
}