use super::{
    MAX_INSTANCES,
    CommonUniform, TwoDInstance,
    util::*,
};

pub struct TwoDPipeline {
    // Buffers
    pub instance_buffer_2d: wgpu::Buffer,

    // Bind Groups
    instanced_bindgroup_layout: wgpu::BindGroupLayout,
    two_d_bind_group: wgpu::BindGroup,

    // Shader Modules
    vs_module_2d: wgpu::ShaderModule,
    fs_module_2d: wgpu::ShaderModule,

    // Pipeline
    instanced_pipeline_layout: wgpu::PipelineLayout,
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

        // Create bind group layouts
        let instanced_bindgroup_layout = create_instanced_bindgroup_layout(device);

        // Create the actual bindgroups
        let two_d_bind_group = create_instanced_bindgroup(
            device, 
            &instanced_bindgroup_layout,
            common_uniform_buffer,
            std::mem::size_of::<CommonUniform>() as wgpu::BufferAddress,
            &instance_buffer_2d,
            instance_buffer_2d_size);

        // Create Pipeline layout
        let instanced_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Instanced Pipeline Layout"),
            push_constant_ranges: &[],
            bind_group_layouts: &[&instanced_bindgroup_layout],
        });

        // Import shaders
        let vs_module_2d = device.create_shader_module(wgpu::include_spirv!("shaders/lines_Vertex.spirv"));
        let fs_module_2d = device.create_shader_module(wgpu::include_spirv!("shaders/lines_Fragment.spirv"));

        // Create pipeline
        let pipeline_2d = create_instanced_pipeline(device, sc_desc, &instanced_pipeline_layout, &vs_module_2d, &fs_module_2d, false);

        Self {
            // Buffers
            instance_buffer_2d,

            // Bind Groups
            instanced_bindgroup_layout,
            two_d_bind_group,

            // Shader Modules
            vs_module_2d,
            fs_module_2d,

            // Pipeline
            instanced_pipeline_layout,
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
            &self.instanced_pipeline_layout, 
            &self.vs_module_2d,
            &self.fs_module_2d,
            false
        );
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