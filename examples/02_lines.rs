use aurora::{
    run, 
    BaseApp,
    Colour,
    GeometryManager,
    LineDescription,
    Point2,
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
        geometry_manager: &mut GeometryManager,
        texture_manger: &mut TextureManager,
    ) -> Self {
        Self {}
    }

    fn handle_input(&mut self, event: WindowEvent) {

    }

    fn update(&mut self, delta_t: f32) {

    }

    fn resize(&mut self) {

    }

    fn draw<R: Renderer>(&mut self, renderer: &mut R) {
        // Clear the screen
        renderer.add(RenderCommand::Clear(
            Colour{
                r: 0.2,
                g: 0.1,
                b: 0.3,
                a: 1.0,
            }
        ));

        // Draw a line
        renderer.add(RenderCommand::DrawLine(LineDescription{
            start: Point2{
                x: 10.0,
                y: 10.0,
            },
            end: Point2{
                x: 200.0,
                y: 50.0,
            },
            width: 20.0,
            colour: Colour{
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
        }))
    }
}

// Start the app
fn main() {
    aurora::run::<Empty>("Lines Example");
}