use aurora::{ 
    BaseApp,
    GeometryManager,
    TextureManager,
    Renderer,
    WindowEvent,
};

// Base structure for the application
struct Empty {

}

// Implement the trait for the main application loop
impl BaseApp for Empty {
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

    fn draw<R: Renderer>(&mut self, _renderer: &mut R) {

    }
}

// Start the app
fn main() {
    aurora::run::<Empty>("Empty Example");
}