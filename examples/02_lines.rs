use aurora::{ 
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
struct Lines {

}

// Implement the trait for the main application loop
impl BaseApp for Lines {
    fn init(
        _geometry_manager: &mut GeometryManager,
        _texture_manager: &mut TextureManager,
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

        // Draw a line
        renderer.add(RenderCommand::DrawLine(LineDescription{
            start: Point2{
                x: 100.0,
                y: 100.0,
            },
            end: Point2{
                x: 400.0,
                y: 300.0,
            },
            width: 1.0,
            colour: Colour{
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        }));

        // Draw a second line
        renderer.add(RenderCommand::DrawLine(LineDescription{
            start: Point2{
                x: 100.0,
                y: 150.0,
            },
            end: Point2{
                x: 400.0,
                y: 350.0,
            },
            width: 1.0,
            colour: Colour{
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            },
        }));
    }
}

// Start the app
fn main() {
    aurora::run::<Lines>("Lines Example");
}