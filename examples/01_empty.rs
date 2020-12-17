use aurora::{
    run, 
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

    }
}

// Start the app
fn main() {
    aurora::run::<Empty>("Empty Example");
}