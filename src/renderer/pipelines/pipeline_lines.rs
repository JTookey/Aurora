use wgpu::util::DeviceExt;

struct LinePipeline {
    // Pipeline Resources
    instance_buffer: wgpu::Buffer,

    // Depth buffers - specific to each pipeline
    depth_buffer_2d: Texture,
    
    // Bind Groups Layouts
    bindgroup_layout: wgpu::BindGroupLayout,

    // Bind groups
    bind_group: wgpu::BindGroup,

    // Rendering pipelines
    pipeline: wgpu::RenderPipeline,
}

impl LinePipeline {
    fn init(
        sc_desc: &wgpu::SwapChainDescriptor,
        device: &wgpu::Device
    ) -> Self {




        Self {

        }
    }
}