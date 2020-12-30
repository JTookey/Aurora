use std::time::{Duration, Instant};

use super::{
    BaseApp, 
    setup,
    Setup,
    TextureManager,
    GeometryManager,
    RendererInstance,
    CommandManager,
    CommandProcessor,
};

use winit::{
    event::{self, Event, WindowEvent},
    event_loop::ControlFlow
};

fn start<App: BaseApp>(
    Setup {
        window,
        event_loop,
        instance,
        size,
        surface,
        adapter,
        device,
        queue,
    }: Setup,
) {
    #[cfg(not(target_arch = "wasm32"))]
    let (mut pool, spawner) = {
        let local_pool = futures::executor::LocalPool::new();
        let spawner = local_pool.spawner();
        (local_pool, spawner)
    };

    #[cfg(target_arch = "wasm32")]
    let spawner = {
        use futures::{future::LocalFutureObj, task::SpawnError};

        struct WebSpawner {}
        impl LocalSpawn for WebSpawner {
            fn spawn_local_obj(
                &self,
                future: LocalFutureObj<'static, ()>,
            ) -> Result<(), SpawnError> {
                Ok(wasm_bindgen_futures::spawn_local(future))
            }
        }

        WebSpawner {}
    };

    log::info!("Initializing the application...");
    let mut texture_manager = TextureManager::new();
    let mut geometry_manager = GeometryManager::new();

    let mut command_manager = CommandManager::new();

    let mut renderer = RendererInstance::new(
        instance,
        size,
        surface,
        adapter,
        device,
        queue,
    );

    let mut main_app = App::init(&mut geometry_manager, &mut texture_manager);

    log::info!("Creating timer...");
    #[cfg(not(target_arch = "wasm32"))]
    let mut last_update_inst = Instant::now();
    let mut delta_t = last_update_inst.elapsed();

    log::info!("Entering render loop...");
    event_loop.run(move |event, _, control_flow| {
        let _ = (&command_manager, &renderer); // force ownership by the closure

        // Set the control flow type based on the target
        *control_flow = if cfg!(feature = "metal-auto-capture") {
            ControlFlow::Exit
        } else {
            #[cfg(not(target_arch = "wasm32"))]
            {
                ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(10))
            }
            #[cfg(target_arch = "wasm32")]
            {
                ControlFlow::Poll
            }
        };

        // Resolve the events
        match event {

            // If everything is done then wait until new frame needed then request
            Event::MainEventsCleared => {
                delta_t = last_update_inst.elapsed();

                #[cfg(not(target_arch = "wasm32"))]
                {
                    if  delta_t > Duration::from_millis(16) {
                        window.request_redraw();
                        last_update_inst = Instant::now();
                    }

                    pool.run_until_stalled();
                }

                #[cfg(target_arch = "wasm32")]
                window.request_redraw();
            }

            // Resolve resize event
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                log::info!("Resizing to {:?}", size);
                renderer.resize(size);
            }

            // Pass inputs to the App
            Event::WindowEvent { event, .. } => match event {

                // TEMP: Close window on "Esc" pushed
                WindowEvent::KeyboardInput {
                    input:
                        event::KeyboardInput {
                            virtual_keycode: Some(event::VirtualKeyCode::Escape),
                            state: event::ElementState::Pressed,
                            ..
                        },
                    ..
                }
                | WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {
                    main_app.handle_input(
                        event
                    );
                }
            },
            Event::RedrawRequested(_) => {
                // Update the app
                main_app.update( delta_t.as_secs_f32() );

                // Start a new frame
                command_manager.clear();
                renderer.init_new_frame();

                // Request app to draw to frame
                let mut cp = CommandProcessor::create(
                    &mut command_manager, 
                    &mut texture_manager,
                );
                main_app.draw(&mut cp);

                // Build and Submit frame to GPU
                renderer.build_and_submit(&command_manager, &texture_manager);
            }
            _ => {}
        }
    });
}


#[cfg(not(target_arch = "wasm32"))]
pub fn run<App: BaseApp>(title: &str) {
    let setup = futures::executor::block_on(setup::<App>(title));
    start::<App>(setup);
}

#[cfg(target_arch = "wasm32")]
pub fn run<App: BaseApp>(title: &str) {
    let title = title.to_owned();
    wasm_bindgen_futures::spawn_local(async move {
        let setup = setup::<App: BaseApp>(&title).await;
        start::<App>(setup);
    });
}