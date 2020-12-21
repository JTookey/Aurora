use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SharedUniform {
    pub texture_position: [f32; 2],
    pub texture_scale: [f32;2],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CommonUniform {
    pub screen_size: [f32;2],
}