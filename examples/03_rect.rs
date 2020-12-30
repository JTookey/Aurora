use aurora::{ 
    BaseApp,
    Colour,
    GeometryManager,
    TwoDDescription,
    Point2,
    Vector2,
    TextureManager,
    Renderer,
    RenderCommand,
    WindowEvent,
};

// Base structure for the application
struct Empty {

}

// Implement the trait for the main application loop
impl BaseApp for Empty {
    fn init(
        _geometry_manager: &mut GeometryManager,
        _texture_manger: &mut TextureManager,
    ) -> Self {
        Self {}
    }

    fn handle_input(&mut self, _event: WindowEvent) {

    }

    fn update(&mut self, _delta_t: f32) {

    }

    fn resize(&mut self) {

    }

    fn draw<R: Renderer>(&mut self, renderer: &mut R) {
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
            size: Vector2::new(200.0,200.0),
            colour: Colour{
                r: 1.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            },
            .. TwoDDescription::default()
        }));
    }
}

// Start the app
fn main() {
    aurora::run::<Empty>("Lines Example");
}