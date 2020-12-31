use crate::{Texture, Vector2};

use super::{
    MAX_INSTANCES,
    CommonUniform, TwoDInstance,
    util::*,
};

pub struct TwoDPipeline {
    // Buffers
    pub instance_buffer_2d: wgpu::Buffer,

    // Bound texture
    pipeline_texture: Texture,

    // Bind Groups
    two_d_bind_group_layout: wgpu::BindGroupLayout,
    two_d_bind_group: wgpu::BindGroup,

    // Shader Modules
    vs_module_2d: wgpu::ShaderModule,
    fs_module_2d: wgpu::ShaderModule,

    // Pipeline
    two_d_pipeline_layout: wgpu::PipelineLayout,
    pipeline_2d: wgpu::RenderPipeline,
}

impl TwoDPipeline {
    pub fn new(
        device: &wgpu::Device,
        sc_desc: &wgpu::SwapChainDescriptor,
        common_uniform_buffer: &wgpu::Buffer,
    ) -> Self {

        // Create instance buffer
        let instance_buffer_2d_size = (MAX_INSTANCES * std::mem::size_of::<TwoDInstance>()) as wgpu::BufferAddress;
        let instance_buffer_2d = device.create_buffer( &wgpu::BufferDescriptor{
            label: Some("Line Instance Buffer"),
            mapped_at_creation: false,
            size: instance_buffer_2d_size,
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
        });

        // Create Initial Pipeline Texture
        let pipeline_texture = Texture::new(device, 256, 256);

        // Create bind group layouts
        let two_d_bind_group_layout = create_main_bind_group_layout(device);

        // Create the actual bindgroups
        let two_d_bind_group = create_main_bind_group(
            device, 
            &two_d_bind_group_layout,
            common_uniform_buffer,
            &pipeline_texture,
        );

        // Create Pipeline layout
        let two_d_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("2D Pipeline Layout"),
            push_constant_ranges: &[],
            bind_group_layouts: &[&two_d_bind_group_layout],
        });

        // Import shaders
        let vs_module_2d = device.create_shader_module(wgpu::include_spirv!("shaders/two_d_pipeline_Vertex.spirv"));
        let fs_module_2d = device.create_shader_module(wgpu::include_spirv!("shaders/two_d_pipeline_Fragment.spirv"));

        // Create pipeline
        let pipeline_2d = create_instanced_pipeline(
            device, 
            sc_desc, 
            &two_d_pipeline_layout,
            TwoDInstance::desc(), 
            &vs_module_2d, 
            &fs_module_2d, 
            false
        );

        Self {
            // Buffers
            instance_buffer_2d,

            // Texture
            pipeline_texture,

            // Bind Groups
            two_d_bind_group_layout,
            two_d_bind_group,

            // Shader Modules
            vs_module_2d,
            fs_module_2d,

            // Pipeline
            two_d_pipeline_layout,
            pipeline_2d,
        }
    }

    // Function to resize the pipeline
    pub fn resize(
        &mut self, 
        device: &wgpu::Device,
        sc_desc: &wgpu::SwapChainDescriptor
    ) {
        // Recreate the pipeline
        self.pipeline_2d = create_instanced_pipeline(
            device, 
            sc_desc, 
            &self.two_d_pipeline_layout,
            TwoDInstance::desc(), 
            &self.vs_module_2d,
            &self.fs_module_2d,
            false
        );
    }

    // Prepare the buffers
    pub fn prepare_buffers(
        &mut self,
        device: &wgpu::Device,
        common_uniform_buffer: &wgpu::Buffer,
        buffer_dimensions_required: Vector2,
    ) {
        let (width, height) = self.pipeline_texture.get_size();
        if buffer_dimensions_required.x > width as f32 || buffer_dimensions_required.y > height as f32 {
            self.pipeline_texture = Texture::new(device, buffer_dimensions_required.x as u32, buffer_dimensions_required.y as u32);
            self.two_d_bind_group = create_main_bind_group(device, &self.two_d_bind_group_layout, common_uniform_buffer, &self.pipeline_texture);
        }
    }

    // Update the instances currently on the GPU
    pub fn update_instance_buffer(
        &self,
        queue: &wgpu::Queue,
        instances: &[TwoDInstance],
    ) {
        queue.write_buffer(
            &self.instance_buffer_2d, 
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
        texture: Option<&Texture>,
        load_op: wgpu::LoadOp<wgpu::Color>,
    ) {

        // Create command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
            label: Some("Instanced Line Command Encoder"),
        });

        // Copy texture to bound resources
        if let Some(tex) = texture {
            encoder.copy_texture_to_texture(
                wgpu::TextureCopyView {
                    texture: tex.get_texture_buffer(),
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: 0,
                    },
                },
                wgpu::TextureCopyView {
                    texture: self.pipeline_texture.get_texture_buffer(),
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: 0,
                    },
                }, 
                tex.get_extent());
        }

        // Create a render pass
        {
            let mut rpass = create_render_pass(
                &mut encoder, 
                frame, 
                None,
                load_op,
            );

            // Set the normal pipeline
            rpass.set_pipeline(&self.pipeline_2d);

            // Set each of the bind groups
            rpass.set_bind_group(0, &self.two_d_bind_group, &[]);

            // Set the instances
            rpass.set_vertex_buffer(0, self.instance_buffer_2d.slice(..));

            // Render
            rpass.draw(0..4, start_instance..end_instance);
        }

        // Complete
        queue.submit(Some(encoder.finish()));
    }
}