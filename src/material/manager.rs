use std::collections::HashMap;

use crate::Vector2;

use super::{
    AssetHolder, 
    InternalHandle, 
    TextureHandle, 
    RawTextureData, 
    SubTexture, 
    Texture,
    util::{
        load_from_file,
        copy_raw_to_gpu,
    }
};

// Texture Manager
pub struct TextureManager {
    sub_texture_map: HashMap<TextureHandle, SubTexture>,
    textures: HashMap<InternalHandle, AssetHolder<Texture,RawTextureData>>,
    next_internal_handle: InternalHandle,
    next_texture_handle: TextureHandle,
    needs_loading: Vec<TextureHandle>,
    needs_preparing: bool,
    max_texture_size: Vector2,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            sub_texture_map: HashMap::new(),
            textures: HashMap::new(),
            next_internal_handle: 0,
            next_texture_handle: 0,
            needs_loading: Vec::new(),
            needs_preparing: false,
            max_texture_size: Vector2::new(256.0, 256.0),
        }
    }

    pub fn buffer_dimensions_required(&self) -> Vector2 {
        self.max_texture_size
    }

    pub fn create_texture_from_data(&mut self, raw_data: Vec<u8>, width: u32, height: u32) -> TextureHandle {
        // Ensure biggest texture dimentions captured
        self.max_texture_size.x = self.max_texture_size.x.max(width as f32);
        self.max_texture_size.y = self.max_texture_size.y.max(height as f32);

        let holder = AssetHolder::Unprepared( RawTextureData{
            width,
            height,
            data: raw_data,
        });
        let i_handle = self.next_internal_handle;
        self.textures.insert(i_handle, holder);
        self.needs_loading.push(i_handle);
        self.next_internal_handle += 1;
        self.needs_preparing = true;

        let sub_texture = SubTexture {
            texture: i_handle,
            texture_position: [0, 0],
            texture_size: [width, height],
        };
        self.add_sub_texture(sub_texture)
    }

    pub fn create_texture_from_file(&mut self, filename: &str) -> TextureHandle {
        let (raw_data, width, height) = load_from_file(filename);
        self.create_texture_from_data(raw_data, width, height)
    }

    pub fn create_sub_texture(&mut self, texture: TextureHandle, pos_x: u32, pos_y: u32, width: u32, height: u32) -> TextureHandle {
        if let Some(parent_sub_tex) = self.sub_texture_map.get(&texture) {
            let is_inside = 
                (pos_x + width)  <= parent_sub_tex.texture_size[0] && 
                (pos_y + height) <= parent_sub_tex.texture_size[1];
            
            if is_inside {
                let new_pos_x = parent_sub_tex.texture_position[0] + pos_x;
                let new_pos_y = parent_sub_tex.texture_position[1] + pos_y;
                
                let new_sub_texture = SubTexture {
                    texture: parent_sub_tex.texture,
                    texture_position: [new_pos_x, new_pos_y],
                    texture_size: [width, height],
                };

                return self.add_sub_texture(new_sub_texture);
            }
        }
        
        0
    }

    fn add_sub_texture(&mut self, new_sub_texture: SubTexture) -> TextureHandle {
        let t_handle = self.next_texture_handle;
        self.next_texture_handle += 1;
        self.sub_texture_map.insert(t_handle, new_sub_texture);

        t_handle
    }

    pub fn needs_preparing(&self) -> bool {
        self.needs_preparing
    }

    pub fn prepare(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        for h in &self.needs_loading {
            if let Some(holder) = self.textures.get_mut(&h) {
                match holder {
                    AssetHolder::Unprepared(raw_texture) => {
                        let (extent, texture, sampler) = copy_raw_to_gpu(device, queue, &raw_texture.data, raw_texture.width, raw_texture.height);
                        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    
                        let texture = Texture {
                            texture_extent: extent,
                            texture_buffer: texture,
                            texture_view: view,
                            texture_sampler: sampler,
                        };

                        *holder = AssetHolder::Loaded(texture);
                    },
                    _ => {},
                }
            }
         }

         self.needs_loading.clear();
         self.needs_preparing = false;
    }

    pub fn get_sub_texture(&self, handle: &TextureHandle) -> Option<&SubTexture> {
        self.sub_texture_map.get(handle)
    }

    pub fn get_texture(&self, handle: &TextureHandle) -> Option<&Texture> {
        let holder = self.textures.get(handle);
        if let Some(holder) = holder {
            if let AssetHolder::Loaded(tex) = holder {
                return Some(tex);
            }
        }
        None
    }

    pub fn drop(&mut self, handle: TextureHandle) {
        self.textures.remove(&handle);
    }
}