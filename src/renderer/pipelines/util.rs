use super::Texture;

pub fn create_render_pass<'frame>(
    encoder: &'frame mut wgpu::CommandEncoder, 
    frame_view: &'frame wgpu::TextureView,
    depth_attachement: std::option::Option<wgpu::RenderPassDepthStencilAttachment<'frame>>,
    load_op: wgpu::LoadOp<wgpu::Color>,
) -> wgpu::RenderPass<'frame> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[wgpu::RenderPassColorAttachment {
            view: &frame_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: load_op,
                store: true,
            },
        }],
        depth_stencil_attachment: depth_attachement,
    })
}

pub fn create_instanced_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    min_binding_size: None,
                    has_dynamic_offset: false,
                },
                count: None,
            },
        ],
    })
}

pub fn create_instanced_bind_group(
    device: &wgpu::Device,
    uniform_bind_group_layout: &wgpu::BindGroupLayout,
    uniform_buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: uniform_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    wgpu::BufferBinding{
                        buffer: uniform_buffer,
                        offset: 0,
                        size: None,
                    }
                ),
            },
        ],
    })
}

pub fn create_instanced_pipeline(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    pipeline_layout: &wgpu::PipelineLayout,
    vertex_buffer_layout: wgpu::VertexBufferLayout,
    shader: &wgpu::ShaderModule,
    depth_checked: bool,
) -> wgpu::RenderPipeline {

    // Define the depth descriptor
    let depth_descriptor = if depth_checked {
        Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState{
                front: wgpu::StencilFaceState::IGNORE,
                back: wgpu::StencilFaceState::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
            bias: wgpu::DepthBiasState { 
                constant: 1, 
                slope_scale: 1.0, 
                clamp: 0.0, 
            },            
        })
    } else {
        None
    };


    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vs_main",
            buffers: &[vertex_buffer_layout],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: "fs_main",
            targets: &[config.format.into()],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleStrip,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        multisample: wgpu::MultisampleState { 
            count: 1, 
            mask: !0, 
            alpha_to_coverage_enabled: false 
        },
        multiview: None,
        depth_stencil: depth_descriptor,
    })
}

pub fn create_main_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    min_binding_size: None,
                    has_dynamic_offset: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture{
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(
                    wgpu::SamplerBindingType::Filtering
                ),
                count: None,
            },
        ],
    })
}

pub fn create_main_bind_group(
    device: &wgpu::Device,
    main_bind_group_layout: &wgpu::BindGroupLayout,
    uniform_buffer: &wgpu::Buffer,
    pipeline_texture: &Texture
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: main_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    wgpu::BufferBinding{
                        buffer: uniform_buffer,
                        offset: 0,
                        size: None,
                    }
                ),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(pipeline_texture.get_view()),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(pipeline_texture.get_sampler()),
            },
        ],
    })
}