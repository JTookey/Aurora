use aurora::{ 
    BaseApp,
    GeometryManager,
    TextureManager,
    Renderer,
    WindowEvent,
    WindowSize,
};

// Base structure for the application
struct Empty {

}

// Implement the trait for the main application loop
impl <'app> BaseApp for Empty {
    fn init(
        _window_size: WindowSize,
        _geometry_manager: &mut GeometryManager,
        _texture_manager: &mut TextureManager,
    ) -> Self {
        Self {}
    }

    fn handle_input(&mut self, _event: WindowEvent) {

    }

    fn update(&mut self, _delta_t: f32) {

    }

    fn resize(&mut self, _size: WindowSize) {

    }

    fn draw<'draw, R: Renderer<'draw>>(&mut self, _renderer: R) {

    }
}

// Start the app
fn main() {
    aurora::run::<Empty>("Empty Example");
}