use super::{
    CommandExecutor,
    CommandManager,
    InstanceManager,
    PipelineManager,
    Renderer,
    RenderCommand,
};

pub struct RendererInstance {
    _instance: wgpu::Instance,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    _adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,

    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,

    frame: Option<wgpu::SwapChainFrame>,

    command_manager: CommandManager,
    instance_manager: InstanceManager,
    pipeline_manager: PipelineManager,
}

impl RendererInstance {
    pub fn new(
        instance: wgpu::Instance,
        size: winit::dpi::PhysicalSize<u32>,
        surface: wgpu::Surface,
        adapter: wgpu::Adapter,
        device: wgpu::Device,
        queue: wgpu::Queue
    ) -> Self {
        // Create the Swap Chain Descriptor
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            // TODO: Allow srgb unconditionally
            format: if cfg!(target_arch = "wasm32") {
                wgpu::TextureFormat::Bgra8Unorm
            } else {
                wgpu::TextureFormat::Bgra8UnormSrgb
            },
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
        };

        // Create the actual Swap Chain
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        // Create the Command Manager
        let command_manager = CommandManager::new();

        // Create Instance Manager
        let instance_manager = InstanceManager::new();

        // Create Pipeline Manager
        let pipeline_manager = PipelineManager::new(&device, &sc_desc);

        // Build and return the Render Instance
        RendererInstance{
            _instance: instance,
            size,
            surface,
            _adapter: adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            frame: None,
            command_manager,
            instance_manager,
            pipeline_manager,
        }
    }

    pub fn init_new_frame(&mut self) {
        // Clear out all previous cmds
        self.command_manager.clear();
        self.instance_manager.clear();

        // Attempt to aquire a new frame
        let frame = match self.swap_chain.get_current_frame() {
            Ok(frame) => frame,
            Err(_) => {
                self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
                self.swap_chain
                    .get_current_frame()
                    .expect("Failed to acquire next swap chain texture!")
            }
        };

        // Provide the frame to be rendered too
        self.frame = Some(frame);
    }

    pub fn build_and_submit(&mut self) {
        // Render on the GPU
        if let Some(frame) = &self.frame {
            
            // Create Command Executor
            let mut ce = CommandExecutor::new(
                &self.device,
                &self.queue, 
                frame, 
                &self.command_manager, 
                &self.instance_manager, 
                &mut self.pipeline_manager);

            // Run
            ce.build_frame();
        }

        // Drop the frame to present it to the Surface
        self.frame.take();
    }

    pub fn resize(&mut self) {
        // Resize the swap chain
        self.sc_desc.width = if self.size.width == 0 { 1 } else { self.size.width };
        self.sc_desc.height = if self.size.height == 0 { 1 } else { self.size.height };
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);

        // Resize the pipelines (i.e. the depth buffers)
    }
}

impl Renderer for RendererInstance {
    fn add(&mut self, cmd: RenderCommand) {
        self.command_manager.process_cmd(cmd, &mut self.instance_manager);
    }
}