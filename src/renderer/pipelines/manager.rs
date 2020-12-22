use super::{CommonUniform, SharedUniform, LineInstance};

use wgpu::util::DeviceExt;
pub const MAX_INSTANCES: usize = 500;
pub const MAX_QUADS_PER_DRAW: usize = 500;

pub struct PipelineManager {
    // Buffers
    pub shared_uniform_buffer: wgpu::Buffer,
    pub common_uniform_buffer: wgpu::Buffer,
    pub instance_buffer_lines: wgpu::Buffer,

    // Bind Groups
    instanced_bindgroup_layout: wgpu::BindGroupLayout,
    line_bind_group: wgpu::BindGroup,

    // Pipeline
    instanced_pipeline_layout: wgpu::PipelineLayout,
    pipeline_line: wgpu::RenderPipeline,
}

impl PipelineManager {
    pub fn new(
        device: &wgpu::Device,
        sc_desc: &wgpu::SwapChainDescriptor,
    ) -> Self {

        // Shared Uniform
        let shared_uniform = SharedUniform {
            texture_position: [0.0, 0.0],
            texture_scale: [1.0, 1.0],
        };

        let shared_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("Shared Uniform Buffer"),
            contents:   bytemuck::bytes_of(&shared_uniform), 
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST
        });

        // Primative Uniform
        let common_uniform = CommonUniform{
            screen_size: [sc_desc.width as f32, sc_desc.height as f32],
        };
        let common_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("Commmon Uniform Buffer"),
            contents: bytemuck::bytes_of(&common_uniform), 
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST
        });

        // Create instance buffer
        let instance_buffer_line_size = (MAX_INSTANCES * std::mem::size_of::<LineInstance>()) as wgpu::BufferAddress;
        let instance_buffer_lines = device.create_buffer( &wgpu::BufferDescriptor{
            label: Some("Line Instance Buffer"),
            mapped_at_creation: false,
            size: instance_buffer_line_size,
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
        });

        // Create bind group layouts
        let instanced_bindgroup_layout = create_instanced_bindgroup_layout(device);

        // Create the actual bindgroups
        let line_bind_group = create_instanced_bindgroup(
            device, 
            &instanced_bindgroup_layout,
            &common_uniform_buffer,
            std::mem::size_of::<CommonUniform>() as wgpu::BufferAddress,
            &instance_buffer_lines,
            instance_buffer_line_size);

        // Create Pipeline layout
        let instanced_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Instanced Pipeline Layout"),
            push_constant_ranges: &[],
            bind_group_layouts: &[&instanced_bindgroup_layout],
        });

        // Import shaders
        let vs_module_line = device.create_shader_module(wgpu::include_spirv!("shaders/lines_Vertex.spirv"));
        let fs_module_line = device.create_shader_module(wgpu::include_spirv!("shaders/lines_Fragment.spirv"));

        // Create pipeline
        let pipeline_line = create_instanced_pipeline(device, sc_desc, &instanced_pipeline_layout, &vs_module_line, &fs_module_line, false);

        Self {
            // Buffers
            shared_uniform_buffer,
            common_uniform_buffer,
            instance_buffer_lines,

            // Bind Groups
            instanced_bindgroup_layout,
            line_bind_group,

            // Pipeline
            instanced_pipeline_layout,
            pipeline_line,
        }
    }

    // Update the instances currently on the GPU
    pub fn update_instance_buffer(
        &self,
        queue: &wgpu::Queue,
        instances: &[LineInstance],
    ) {
        queue.write_buffer(
            &self.instance_buffer_lines, 
            0, 
            bytemuck::cast_slice(instances),
        );
    }

    pub fn render_instances(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        frame: &wgpu::SwapChainFrame,
        start_instance: u32,
        end_instance: u32,
        load_op: wgpu::LoadOp<wgpu::Color>,
    ) {

        // Create command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
            label: Some("Instanced Line Command Encoder"),
        });

        // Create a render pass
        {
            let mut rpass = create_render_pass(
                &mut encoder, 
                frame, 
                None,
                load_op,
            );

            // Set the normal pipeline
            rpass.set_pipeline(&self.pipeline_line);

            // Set each of the bind groups
            rpass.set_bind_group(0, &self.line_bind_group, &[]);

            // Set the instances
            rpass.set_vertex_buffer(0, self.instance_buffer_lines.slice(..));

            // Render
            rpass.draw(0..4, start_instance..end_instance);
        }

        // Complete
        queue.submit(Some(encoder.finish()));
    }
}

fn create_render_pass<'frame>(
    encoder: &'frame mut wgpu::CommandEncoder, 
    frame: &'frame wgpu::SwapChainFrame,
    depth_attachement: std::option::Option<wgpu::RenderPassDepthStencilAttachmentDescriptor<'frame>>,
    load_op: wgpu::LoadOp<wgpu::Color>,
) -> wgpu::RenderPass<'frame> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
            attachment: &frame.output.view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: load_op,
                store: true,
            },
        }],
        depth_stencil_attachment: depth_attachement,
    })
}

fn create_instanced_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { 
                    min_binding_size: None,
                    dynamic: false,
                },
                count: None,
            },
            // wgpu::BindGroupLayoutEntry {
            //     binding: 1,
            //     visibility: wgpu::ShaderStage::VERTEX,
            //     ty: wgpu::BindingType::StorageBuffer {
            //         dynamic: false,
            //         readonly: true,
            //         min_binding_size: None,
            //     },
            //     count: None,
            // }
        ],
    })
}

fn create_instanced_bindgroup(
    device: &wgpu::Device,
    uniform_bind_group_layout: &wgpu::BindGroupLayout,
    uniform_buffer: &wgpu::Buffer,
    uniform_size: wgpu::BufferAddress,
    instance_buffer: &wgpu::Buffer, 
    buffer_size: wgpu::BufferAddress
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: uniform_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    uniform_buffer.slice(..)
                ),
            },
            // wgpu::BindGroupEntry {
            //     binding: 1,
            //     resource: wgpu::BindingResource::Buffer(
            //         instance_buffer.slice(..)
            //     ),
            // }
        ],
    })
}

fn create_instanced_pipeline(
    device: &wgpu::Device,
    sc_desc: &wgpu::SwapChainDescriptor,
    pipeline_layout: &wgpu::PipelineLayout,
    vertex_shader: &wgpu::ShaderModule,
    fragment_shader: &wgpu::ShaderModule,
    depth_checked: bool,
) -> wgpu::RenderPipeline {

    // Define the depth descriptor
    let depth_descriptor = if depth_checked {
        Some(wgpu::DepthStencilStateDescriptor {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilStateDescriptor{
                front: wgpu::StencilStateFaceDescriptor::IGNORE,
                back: wgpu::StencilStateFaceDescriptor::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },            
        })
    } else {
        None
    };


    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(pipeline_layout),
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: vertex_shader,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: fragment_shader,
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::Back,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
            clamp_depth: false,
        }),
        primitive_topology: wgpu::PrimitiveTopology::TriangleStrip,
        color_states: &[wgpu::ColorStateDescriptor {
            format: sc_desc.format,
            color_blend: wgpu::BlendDescriptor {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha_blend: wgpu::BlendDescriptor {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::DstAlpha,
                operation: wgpu::BlendOperation::Max,
            },
            write_mask: wgpu::ColorWrite::ALL,
        }],
        depth_stencil_state: depth_descriptor,
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[
                LineInstance::desc(),
            ],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    })
}