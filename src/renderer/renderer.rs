use super::{
    CommandExecutor,
    CommandManager,
    PipelineManager,
    TextureManager,
    SectionManager,
};

pub struct RendererInstance {
    _instance: wgpu::Instance,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    _adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,

    config: wgpu::SurfaceConfiguration,

    frame: Option<wgpu::SurfaceTexture>,

    pipeline_manager: PipelineManager,
}

impl RendererInstance {
    pub fn new(
        instance: wgpu::Instance,
        size: winit::dpi::PhysicalSize<u32>,
        surface: wgpu::Surface,
        adapter: wgpu::Adapter,
        device: wgpu::Device,
        queue: wgpu::Queue,
    ) -> Self {

        // Get the preferred format
        let swapchain_format = surface.get_preferred_format(&adapter).unwrap();

        // Create the Swap Chain Descriptor
        let config = wgpu::SurfaceConfiguration  {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            // TODO: Allow srgb unconditionally
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
        };

        surface.configure(&device, &config);

        // Create Pipeline Manager
        let pipeline_manager = PipelineManager::new(&device, &config);

        // Build and return the Render Instance
        RendererInstance{
            _instance: instance,
            size,
            surface,
            _adapter: adapter,
            device,
            queue,
            config,
            frame: None,
            pipeline_manager,
        }
    }

    pub fn init_new_frame(&mut self) {
        // Attempt to aquire a new frame
        let frame = if let Ok(frame) = self.surface.get_current_texture() {
            frame
        } else {
            self.surface.configure(&self.device, &self.config);
            self.surface.get_current_texture().expect("Failed to acquire next swap chain frame!")
        };

        // Provide the frame to be rendered too
        self.frame = Some(frame);
    }

    pub fn build_and_submit<'frame>(&mut self, command_manager: &CommandManager, section_manger:&mut SectionManager<'frame>, texture_manager: &mut TextureManager) {
        // Render on the GPU
        if let Some(frame) = &self.frame {

            let frame_view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
            
            // Create Command Executor
            let mut ce = CommandExecutor::new(
                &self.device,
                &self.queue, 
                frame,
                &frame_view,
                command_manager,
                section_manger,
                &mut self.pipeline_manager,
                texture_manager,
            );

            // Run
            ce.build_frame();
        }

        // Drop the frame to present it to the Surface
        self.frame.take().unwrap().present();
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        // Update the physical size
        self.size = new_size;

        // Resize the swap chain
        self.config.width = if self.size.width == 0 { 1 } else { self.size.width };
        self.config.height = if self.size.height == 0 { 1 } else { self.size.height };

        // Resize the pipelines (i.e. the depth buffers)
        self.pipeline_manager.resize(
            &self.device, 
            &self.queue, 
            &self.config
        );
    }
}