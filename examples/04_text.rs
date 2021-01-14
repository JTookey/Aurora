use aurora::{ 
    BaseApp,
    Colour,
    GeometryManager,
    TextureManager,
    Renderer,
    RenderCommand,
    WindowEvent,
    Section,
    Text,
    WindowSize,
};

// Base structure for the application
struct TextExample {
    text: String,
}

// Implement the trait for the main application loop
impl BaseApp for TextExample {
    fn init(
        _window_size: WindowSize,
        _geometry_manager: &mut GeometryManager,
        _texture_manager: &mut TextureManager,
    ) -> Self {

        Self {
            text: String::from("Text Example")
        }
    }

    fn handle_input(&mut self, _event: WindowEvent) {

    }

    fn update(&mut self, _delta_t: f32) {
        
    }

    fn resize(&mut self, _size: WindowSize) {

    }

    fn draw<'draw, R: Renderer<'draw>>(&'draw mut self, mut renderer: R) {
        // Clear the screen
        renderer.add(RenderCommand::Clear(
            Colour{
                r: 0.1,
                g: 0.05,
                b: 0.15,
                a: 1.0,
            }
        ));

        // Add text
        renderer.add(RenderCommand::DrawText(
            Section::default()
                .add_text(
                    Text::new(&self.text)
                    .with_scale(30.0)
                    .with_color(
                        [1.0, 1.0, 0.0, 1.0]
                    )
                )
                .with_screen_position((200.0, 600.0))
        ));

        // Add text
        renderer.add(RenderCommand::DrawText(
            Section::default()
            .add_text(
                Text::new("Hello SPACE!!")
                    .with_scale(30.0)
                    .with_color(
                        [1.0, 1.0, 0.0, 1.0]
                    )
            )
            .with_screen_position((200.0, 630.0))
        ));
    }
}

// Start the app
fn main() {
    aurora::run::<TextExample>("Text Example");
}