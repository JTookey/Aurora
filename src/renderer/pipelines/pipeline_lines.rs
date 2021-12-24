use std::borrow::Cow;

use super::{
    MAX_INSTANCES,
    LineInstance,
    util::*,
};

pub struct LinesPipeline {
    // Buffers
    pub instance_buffer_lines: wgpu::Buffer,

    // Bind Groups
    _instanced_bindgroup_layout: wgpu::BindGroupLayout,
    line_bind_group: wgpu::BindGroup,

    // Shader Modules
    module_line: wgpu::ShaderModule,

    // Pipeline
    instanced_pipeline_layout: wgpu::PipelineLayout,
    pipeline_line: wgpu::RenderPipeline,
}

impl LinesPipeline {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        common_uniform_buffer: &wgpu::Buffer,
    ) -> Self {

        // Create instance buffer
        let instance_buffer_line_size = (MAX_INSTANCES * std::mem::size_of::<LineInstance>()) as wgpu::BufferAddress;
        let instance_buffer_lines = device.create_buffer( &wgpu::BufferDescriptor{
            label: Some("Line Instance Buffer"),
            mapped_at_creation: false,
            size: instance_buffer_line_size,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        // Create bind group layouts
        let instanced_bindgroup_layout = create_instanced_bind_group_layout(device);

        // Create the actual bindgroups
        let line_bind_group = create_instanced_bind_group(
            device, 
            &instanced_bindgroup_layout,
            common_uniform_buffer
        );

        // Create Pipeline layout
        let instanced_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Instanced Pipeline Layout"),
            push_constant_ranges: &[],
            bind_group_layouts: &[&instanced_bindgroup_layout],
        });

        // Import shaders
        let module_line = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Lines Pipeline Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/lines_pipeline.wgsl"))),
        });
        //let fs_module_line = device.create_shader_module(&wgpu::include_spirv!("shaders/lines_Fragment.spirv"));

        // Create pipeline
        let pipeline_line = create_instanced_pipeline(
            device,
            config,
            &instanced_pipeline_layout,
            LineInstance::desc(),
            &module_line, 
            false
        );

        Self {
            // Buffers
            instance_buffer_lines,

            // Bind Groups
            _instanced_bindgroup_layout: instanced_bindgroup_layout,
            line_bind_group,

            // Shader Modules
            module_line,

            // Pipeline
            instanced_pipeline_layout,
            pipeline_line,
        }
    }

    // Function to resize the pipeline
    pub fn resize(
        &mut self, 
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration
    ) {
        // Recreate the pipeline
        self.pipeline_line = create_instanced_pipeline(
            device,
            config,
            &self.instanced_pipeline_layout,
            LineInstance::desc(),
            &self.module_line,
            false
        );
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
        frame_view: &wgpu::TextureView,
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
                frame_view, 
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