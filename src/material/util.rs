pub fn load_from_file(filename: &str) -> (Vec<u8>, u32, u32) {
    let mut buffer: Vec<u8> = Vec::new();

    let texture_file = std::fs::File::open(filename)
        .expect("Couldn't open texture...");
    let decoder = png::Decoder::new( texture_file );

    let mut reader = decoder.read_info()
        .expect("Can't read info...");
        
    buffer.resize(reader.output_buffer_size(), 0);

    reader.next_frame(&mut buffer)
        .expect("Can't read PNG frame...");

    (buffer, reader.info().width, reader.info().height)
}

pub fn create_gpu_texture(device: &wgpu::Device, width: u32, height: u32) -> (wgpu::Extent3d, wgpu::Texture, wgpu::Sampler) {
    // Create Extent
    let texture_extent = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    // Create the Texture Buffer
    let texture_buffer = device.create_texture( &wgpu::TextureDescriptor {
        label: Some("Texture Buffer"),
        size: texture_extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST,
    });

    // Create Sampler
    let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Texture Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: None,
            anisotropy_clamp: None,
            border_color: None,
    });

    (texture_extent, texture_buffer, texture_sampler)
}

pub fn copy_raw_to_gpu(device: &wgpu::Device, queue: &wgpu::Queue, raw_data: &Vec<u8>, width: u32, height: u32) -> (wgpu::Extent3d, wgpu::Texture, wgpu::Sampler) {
    // Create the texture on the gpu
    let (texture_extent, texture_buffer, texture_sampler) = create_gpu_texture(device, width, height);

    // Write the Texture Buffer
    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture_buffer,
            mip_level: 0,
            origin: wgpu::Origin3d {
                x: 0,
                y: 0,
                z: 0,
            },
            aspect: wgpu::TextureAspect::All,
        },
        raw_data, 
        wgpu::ImageDataLayout{
            offset: 0,
            bytes_per_row: core::num::NonZeroU32::new(4 * width),
            rows_per_image: core::num::NonZeroU32::new(height),
        }, 
        texture_extent,
    );

    (
        texture_extent,
        texture_buffer,
        texture_sampler,
    )
}