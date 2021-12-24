use std::borrow::Cow;

use wgpu::SurfaceConfiguration;

use crate::{Texture, Vector2};

use super::{
    MAX_INSTANCES,
    TwoDInstance,
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
    module_2d: wgpu::ShaderModule,

    // Pipeline
    two_d_pipeline_layout: wgpu::PipelineLayout,
    pipeline_2d: wgpu::RenderPipeline,
}

impl TwoDPipeline {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        common_uniform_buffer: &wgpu::Buffer,
    ) -> Self {

        // Create instance buffer
        let instance_buffer_2d_size = (MAX_INSTANCES * std::mem::size_of::<TwoDInstance>()) as wgpu::BufferAddress;
        let instance_buffer_2d = device.create_buffer( &wgpu::BufferDescriptor{
            label: Some("Line Instance Buffer"),
            mapped_at_creation: false,
            size: instance_buffer_2d_size,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
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
        let module_2d = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("2D Pipeline Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/two_d_pipeline.wgsl"))),
        });
        //let fs_module_2d = device.create_shader_module(&wgpu::include_spirv!("shaders/two_d_pipeline_Fragment.spirv"));

        // Create pipeline
        let pipeline_2d = create_instanced_pipeline(
            device, 
            config,
            &two_d_pipeline_layout,
            TwoDInstance::desc(),
            &module_2d, 
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
            module_2d,

            // Pipeline
            two_d_pipeline_layout,
            pipeline_2d,
        }
    }

    // Function to resize the pipeline
    pub fn resize(
        &mut self, 
        device: &wgpu::Device,
        config: &SurfaceConfiguration,
    ) {
        // Recreate the pipeline
        self.pipeline_2d = create_instanced_pipeline(
            device,
            config,
            &self.two_d_pipeline_layout,
            TwoDInstance::desc(),
            &self.module_2d,
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
        frame_view: &wgpu::TextureView,
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
                wgpu::ImageCopyTexture {
                    texture: tex.get_texture_buffer(),
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: 0,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::ImageCopyTexture {
                    texture: self.pipeline_texture.get_texture_buffer(),
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: 0,
                    },
                    aspect: wgpu::TextureAspect::All,
                }, 
                tex.get_extent());
        }

        // Create a render pass
        {
            let mut rpass = create_render_pass(
                &mut encoder, 
                frame_view, 
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