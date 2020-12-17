use zerocopy::AsBytes;

#[repr(C)]
#[derive(AsBytes)]
pub struct SharedUniform {
    texture_position: [f32; 2],
    texture_scale: [f32;2],
}

#[repr(C)]
#[derive(AsBytes)]
pub struct CommonUniform {
    screen_size: [f32;2],
}