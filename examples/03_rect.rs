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
    WindowSize,
};

// Base structure for the application
struct Rectangles {
    squares_texture: TextureHandle,
    bricks_texture: TextureHandle,
    rotation: f32,
}

// Implement the trait for the main application loop
impl BaseApp for Rectangles {
    fn init(
        _window_size: WindowSize,
        _geometry_manager: &mut GeometryManager,
        texture_manager: &mut TextureManager,
    ) -> Self {

        // Load a texture
        let squares_texture = texture_manager.create_texture_from_file("resources/texture/Squares.png");
        let bricks_texture = texture_manager.create_sub_texture(squares_texture, 32, 0, 32, 32);

        Self {
            squares_texture,
            bricks_texture,
            rotation: 0.0,
        }
    }

    fn handle_input(&mut self, _event: WindowEvent) {

    }

    fn update(&mut self, delta_t: f32) {
        self.rotation += delta_t * 0.5;
    }

    fn resize(&mut self, _size: WindowSize) {

    }

    fn draw<'draw, R: Renderer<'draw>>(&mut self, mut renderer: R) {
        // Clear the screen
        renderer.add(RenderCommand::Clear(
            Colour{
                r: 0.1,
                g: 0.05,
                b: 0.15,
                a: 1.0,
            }
        ));

        // Draw a rectangle
        renderer.add(RenderCommand::Draw2D(TwoDDescription{
            position: Point2::new(100.0,100.0),
            size: Vector2::new(150.0,150.0),
            colour: Colour{
                r: 1.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            },
            opacity: 0.5,
            .. TwoDDescription::default()
        }));

        // Draw a rectangle
        renderer.add(RenderCommand::Draw2D(TwoDDescription{
            position: Point2::new(150.0,150.0),
            size: Vector2::new(150.0,150.0),
            colour: Colour{
                r: 0.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            opacity: 0.5,
            .. TwoDDescription::default()
        }));

        // Draw a textured rectangle
        renderer.add(RenderCommand::Draw2D(TwoDDescription{
            position: Point2::new(400.0,100.0),
            size: Vector2::new(200.0,200.0),
            texture: Some(self.squares_texture),
            corner_radius: 0.2,
            .. TwoDDescription::default()
        }));

        // Draw a textured rectangle
        renderer.add(RenderCommand::Draw2D(TwoDDescription{
            position: Point2::new(700.0,100.0),
            size: Vector2::new(200.0,200.0),
            texture: Some(self.bricks_texture),
            rotation: self.rotation,
            .. TwoDDescription::default()
        }));

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
    aurora::run::<Rectangles>("Rectangles Example");
}