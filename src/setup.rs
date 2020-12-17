use super::BaseApp;

use winit::event_loop::EventLoop;

pub struct Setup {
    pub window: winit::window::Window,
    pub event_loop: EventLoop<()>,
    pub instance: wgpu::Instance,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

pub async fn setup<App: BaseApp>(title: &str) -> Setup {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let chrome_tracing_dir = std::env::var("WGPU_CHROME_TRACE");
        subscriber::initialize_default_subscriber(
            chrome_tracing_dir.as_ref().map(std::path::Path::new).ok(),
        );
    };

    let event_loop = EventLoop::new();
    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title(title);
    #[cfg(windows_OFF)] // TODO
    {
        use winit::platform::windows::WindowBuilderExtWindows;
        builder = builder.with_no_redirection_bitmap(true);
    }
    let window = builder.build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        console_log::init().expect("could not initialize logger");
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        // On wasm, append the canvas to the document body
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
    }

    log::info!("Initializing the surface...");

    let backend = if let Ok(backend) = std::env::var("WGPU_BACKEND") {
        match backend.to_lowercase().as_str() {
            "vulkan" => wgpu::BackendBit::VULKAN,
            "metal" => wgpu::BackendBit::METAL,
            "dx12" => wgpu::BackendBit::DX12,
            "dx11" => wgpu::BackendBit::DX11,
            "gl" => wgpu::BackendBit::GL,
            "webgpu" => wgpu::BackendBit::BROWSER_WEBGPU,
            other => panic!("Unknown backend: {}", other),
        }
    } else {
        wgpu::BackendBit::PRIMARY
    };
    let power_preference = if let Ok(power_preference) = std::env::var("WGPU_POWER_PREF") {
        match power_preference.to_lowercase().as_str() {
            "low" => wgpu::PowerPreference::LowPower,
            "high" => wgpu::PowerPreference::HighPerformance,
            other => panic!("Unknown power preference: {}", other),
        }
    } else {
        wgpu::PowerPreference::default()
    };
    let instance = wgpu::Instance::new(backend);
    let (size, surface) = unsafe {
        let size = window.inner_size();
        let surface = instance.create_surface(&window);
        (size, surface)
    };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference,
            compatible_surface: Some(&surface),
        })
        .await
        .unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    {
        let adapter_info = adapter.get_info();
        println!("Using {} ({:?})", adapter_info.name, adapter_info.backend);
    }

    let optional_features = App::optional_features();
    let required_features = App::required_features();
    let adapter_features = adapter.features();
    assert!(
        adapter_features.contains(required_features),
        "Adapter does not support required features for this example: {:?}",
        required_features - adapter_features
    );

    let needed_limits = App::required_limits();

    let trace_dir = std::env::var("WGPU_TRACE");
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: (optional_features & adapter_features) | required_features,
                limits: needed_limits,
                shader_validation: false,
            },
            trace_dir.ok().as_ref().map(std::path::Path::new),
        )
        .await
        .unwrap();

    Setup {
        window,
        event_loop,
        instance,
        size,
        surface,
        adapter,
        device,
        queue,
    }
}