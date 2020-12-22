use super::{CommonUniform, SharedUniform, LineInstance, LinesPipeline};

use wgpu::util::DeviceExt;
pub const MAX_INSTANCES: usize = 500;
pub const MAX_QUADS_PER_DRAW: usize = 500;

pub struct PipelineManager {
    // Buffers
    pub shared_uniform_buffer: wgpu::Buffer,
    pub common_uniform_buffer: wgpu::Buffer,

    // Pipeline
    pipeline_lines: LinesPipeline,
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

        // Common Uniform
        let common_uniform = CommonUniform{
            screen_size: [sc_desc.width as f32, sc_desc.height as f32],
        };
        let common_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("Commmon Uniform Buffer"),
            contents: bytemuck::bytes_of(&common_uniform), 
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST
        });

        
        // Create pipeline
        let pipeline_lines = LinesPipeline::new(
            device, 
            sc_desc, 
            &common_uniform_buffer
        );

        Self {
            // Buffers
            shared_uniform_buffer,
            common_uniform_buffer,

            // Pipelines
            pipeline_lines,
        }
    }

    // Resize the pipelines
    pub fn resize(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        sc_desc: &wgpu::SwapChainDescriptor,
    ) {
        // Resize the pipelines
        self.pipeline_lines.resize(
            device,
            sc_desc,
        );

        // Create an updated uniform buffer
        let common_uniform = CommonUniform{
            screen_size: [sc_desc.width as f32, sc_desc.height as f32],
        };

        // Write the uniform
        queue.write_buffer(
            &self.common_uniform_buffer, 
            0, 
            bytemuck::bytes_of(&common_uniform));
    }

    // Update the line instances currently on the GPU
    pub fn update_line_instances(
        &self,
        queue: &wgpu::Queue,
        instances: &[LineInstance],
    ) {
        self.pipeline_lines.update_instance_buffer(
            queue,
            instances,
        )
    }

    // Method for rendering lines - pass on the command to the Lines Pipeline 
    pub fn render_lines(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        frame: &wgpu::SwapChainFrame,
        start_instance: u32,
        end_instance: u32,
        load_op: wgpu::LoadOp<wgpu::Color>,
    ) {
        self.pipeline_lines.render_instances(
            device,
            queue,
            frame,
            start_instance,
            end_instance,
            load_op,
        );
    }
}