use aurora::{ 
    BaseApp,
    Colour,
    GeometryManager,
    TwoDDescription,
    Point2,
    Vector2,
    TextureManager,
    TextureHandle,
    Renderer,
    RenderCommand,
    WindowEvent,
    Section,
    Text,
};

// Base structure for the application
struct TextExample {
}

// Implement the trait for the main application loop
impl <'app> BaseApp<'app> for TextExample {
    fn init(
        _geometry_manager: &mut GeometryManager,
        _texture_manager: &mut TextureManager,
    ) -> Self {

        Self {

        }
    }

    fn handle_input(&mut self, _event: WindowEvent) {

    }

    fn update(&mut self, _delta_t: f32) {
        
    }

    fn resize(&mut self) {

    }

    fn draw<R: Renderer<'app>>(&mut self, renderer: &mut R) {
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
                    Text::new("Hello World!")
                    .with_scale(30.0)
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