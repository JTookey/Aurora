use super::Section;
use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder};

const FONT_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;
const DEFAULT_FONT: &'static [u8] = include_bytes!("../../../resources/font/Comfortaa-Regular.ttf");

pub struct TextPipeline {
    glyphbrush: GlyphBrush<()>,
    staging_belt: wgpu::util::StagingBelt,
    local_pool: futures::executor::LocalPool,
    local_spawner: futures::executor::LocalSpawner,
    canvas_size: (u32, u32),
}

impl TextPipeline {
    pub fn new(device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor) -> Self {

        // Read the font
        let glyphs = ab_glyph::FontArc::try_from_slice(DEFAULT_FONT).unwrap();
    
        // Create the glyphbrush
        let glyphbrush = GlyphBrushBuilder::using_font(glyphs)
            .build(&device, FONT_FORMAT);

        // Create a stating belt - I'm assuming this is bytes in size... may need more...
        let staging_belt = wgpu::util::StagingBelt::new(1024);


        let local_pool = futures::executor::LocalPool::new();
        let local_spawner = local_pool.spawner();

        // Local canvas size
        let canvas_size = (sc_desc.width, sc_desc.height);

        Self {
            glyphbrush,
            staging_belt,
            local_pool,
            local_spawner,
            canvas_size,
        }
    }

    pub fn resize(&mut self, sc_desc: &wgpu::SwapChainDescriptor) {
        self.canvas_size = (sc_desc.width, sc_desc.height);
    }

    pub fn render_sections(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        frame: &wgpu::SwapChainFrame,
        sections: &mut [Option<Section>],
    ) {
        // Create command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
            label: Some("Text Command Encoder"),
        });

        // Loop through the sections and queue if valid
        for section_option in sections.iter_mut() {
            if let Some(section) = section_option.take() {
                self.glyphbrush.queue(section);
            }
        }

        // Draw the text!
        self.glyphbrush
            .draw_queued(
                &device,
                &mut self.staging_belt,
                &mut encoder,
                &frame.output.view,
                self.canvas_size.0,
                self.canvas_size.1,
            )
            .expect("Draw queued");
        
        self.staging_belt.finish();
        // Complete
        queue.submit(Some(encoder.finish()));

        // Recall unused staging buffers
        use futures::task::SpawnExt;

        self.local_spawner
            .spawn(self.staging_belt.recall())
            .expect("Recall staging belt");

        self.local_pool.run_until_stalled();
    }
}