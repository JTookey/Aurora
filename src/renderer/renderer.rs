use super::{
    CommandManager,
    Renderer,
};

pub struct RendererInstance {
    instance: wgpu::Instance,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,

    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,

    frame: Option<wgpu::SwapChainFrame>,

    command_manager: CommandManager,
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
            present_mode: wgpu::PresentMode::Mailbox,
        };

        // Create the actual Swap Chain
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        // Create the Command Manager
        let command_manager = CommandManager::new();

        // Build and return the Render Instance
        RendererInstance{
            instance,
            size,
            surface,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            frame: None,
            command_manager,
        }
    }

    pub fn init_new_frame(&mut self) {
        let frame = match self.swap_chain.get_current_frame() {
            Ok(frame) => frame,
            Err(_) => {
                self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
                self.swap_chain
                    .get_current_frame()
                    .expect("Failed to acquire next swap chain texture!")
            }
        };

        self.frame = Some(frame);
    }

    pub fn build_and_submit(&mut self) {
        // Render on the GPU
        if let Some(frame) = &self.frame {
            
        }

        // Drop the frame to present it to the Surface
        self.frame.take();
    }

    pub fn resize(&mut self) {
        self.sc_desc.width = if self.size.width == 0 { 1 } else { self.size.width };
        self.sc_desc.height = if self.size.height == 0 { 1 } else { self.size.height };
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }
}

impl Renderer for RendererInstance {

}