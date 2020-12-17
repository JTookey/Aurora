struct LinePipeline {
    // Uniform buffers
    // Shared
    shared_uniform: SharedUniform,
    shared_uniform_buffer: wgpu::Buffer,

    // 2D pipeline
    common_uniform: CommonUniform,
    common_uniform_buffer: wgpu::Buffer,

    // Pipeline Resources
    instance_buffer: wgpu::Buffer,

    // Depth buffers - specific to each pipeline
    depth_buffer_2d: Texture,
    depth_buffer_3d: Texture,

    // Bind Groups Layouts
    bindgroup_layout: wgpu::BindGroupLayout,

    // Bind groups
    bind_group: wgpu::BindGroup,

    // Rendering pipelines
    pipeline: wgpu::RenderPipeline,
}