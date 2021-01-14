use crate::Vector2;
use super::{CommonUniform, SharedUniform, LineInstance, LinesPipeline, TwoDInstance, TwoDPipeline, Texture, TextPipeline, Section};

use wgpu::util::DeviceExt;
pub const MAX_INSTANCES: usize = 500;

pub struct PipelineManager {
    // Buffers
    pub shared_uniform_buffer: wgpu::Buffer,
    pub common_uniform_buffer: wgpu::Buffer,

    // Pipeline
    pipeline_lines: LinesPipeline,
    pipeline_2d: TwoDPipeline,
    pipeline_text: TextPipeline,
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

        let pipeline_2d = TwoDPipeline::new(
            device, 
            sc_desc, 
            &common_uniform_buffer
        );

        let pipeline_text = TextPipeline::new(
            device,
            sc_desc,
        );

        Self {
            // Buffers
            shared_uniform_buffer,
            common_uniform_buffer,

            // Pipelines
            pipeline_lines,
            pipeline_2d,
            pipeline_text,
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
        self.pipeline_2d.resize(
            device,
            sc_desc,
        );
        self.pipeline_text.resize(
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
    
    // Prepare the buffers
    pub fn prepare_buffers(
        &mut self,
        device: &wgpu::Device,
        buffer_dimensions_required: Vector2,
    ) {
        self.pipeline_2d.prepare_buffers(
            device,
            &self.common_uniform_buffer,
            buffer_dimensions_required,
        );
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

    // Update the line instances currently on the GPU
    pub fn update_two_d_instances(
        &self,
        queue: &wgpu::Queue,
        instances: &[TwoDInstance],
    ) {
        self.pipeline_2d.update_instance_buffer(
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

    // Method for rendering 2D objects - pass on the command to the 2D Pipeline
    pub fn render_2d(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        frame: &wgpu::SwapChainFrame,
        start_instance: u32,
        end_instance: u32,
        texture: Option<&Texture>,
        load_op: wgpu::LoadOp<wgpu::Color>,
    ) {
        self.pipeline_2d.render_instances(
            device,
            queue,
            frame,
            start_instance,
            end_instance,
            texture,
            load_op,
        );
    }

    // Method for rendering text - pass on the command to the Text Pipeline
    pub fn render_sections(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        frame: &wgpu::SwapChainFrame,
        sections: &mut [Option<Section>],
    ) {
        self.pipeline_text.render_sections(
            device,
            queue,
            frame,
            sections,
        );
    }
}