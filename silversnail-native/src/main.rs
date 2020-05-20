use std::fs::File;
use wgpu::*;

static TEXTURE_SIZE: (u32, u32) = (512, 512);

async fn get_adapter() -> Adapter {
    println!("Enumerating available primary adapters...");
    println!(
        "{:#?}",
        Adapter::enumerate(BackendBit::PRIMARY)
            .iter()
            .map(Adapter::get_info)
            .collect::<Vec<_>>()
    );

    let adapter = Adapter::request(
        &RequestAdapterOptions {
            power_preference: PowerPreference::Default,
            compatible_surface: None,
        },
        BackendBit::PRIMARY,
    )
    .await
    .expect("Couldn't find a suitable Adapter.");

    println!("Using: {:#?}", adapter.get_info());
    adapter
}

async fn get_device(adapter: &Adapter) -> (Device, Queue) {
    adapter.request_device(&DeviceDescriptor::default()).await
}

fn create_shaders(device: &Device) -> (ShaderModule, ShaderModule) {
    let vs = include_bytes!("shader.vert.spv");
    let fs = include_bytes!("shader.frag.spv");

    (
        device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&vs[..])).unwrap()),
        device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&fs[..])).unwrap()),
    )
}

fn create_pipeline(device: &Device) -> (RenderPipeline, BindGroup) {
    let (vs_module, fs_module) = create_shaders(&device);

    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        bindings: &[],
        label: None,
    });

    let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        bind_group_layouts: &[&bind_group_layout],
    });

    let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        layout: &layout,
        vertex_stage: ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(ProgrammableStageDescriptor {
            module: &fs_module,
            entry_point: "main",
        }),
        rasterization_state: None,
        primitive_topology: PrimitiveTopology::TriangleList,
        color_states: &[ColorStateDescriptor {
            format: TextureFormat::Rgba8Uint,
            alpha_blend: BlendDescriptor::REPLACE,
            color_blend: BlendDescriptor::REPLACE,
            write_mask: ColorWrite::ALL,
        }],
        depth_stencil_state: None,
        vertex_state: VertexStateDescriptor {
            index_format: IndexFormat::Uint16,
            vertex_buffers: &[],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });

    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        layout: &bind_group_layout,
        bindings: &[],
        label: None,
    });

    (pipeline, bind_group)
}

fn create_output_texture(device: &Device) -> (Texture, TextureView) {
    let texture = device.create_texture(&TextureDescriptor {
        label: None,
        size: Extent3d {
            width: TEXTURE_SIZE.0,
            height: TEXTURE_SIZE.1,
            depth: 1,
        },
        array_layer_count: 1,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba8Uint,
        usage: TextureUsage::OUTPUT_ATTACHMENT | TextureUsage::COPY_SRC,
    });

    let view = texture.create_default_view();

    (texture, view)
}

fn create_output_buffer(device: &Device) -> Buffer {
    let buffer = device.create_buffer(&BufferDescriptor {
        label: None,
        size: TEXTURE_SIZE.0 as u64 * TEXTURE_SIZE.1 as u64 * 4,
        usage: BufferUsage::COPY_DST | BufferUsage::MAP_READ,
    });

    buffer
}

fn render_frame(
    device: &Device,
    queue: &Queue,
    out_view: &TextureView,
    pipeline: &RenderPipeline,
    bind_group: &BindGroup,
    texture: &Texture,
    buffer: &Buffer,
) {
    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor { label: None });

    {
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            color_attachments: &[RenderPassColorAttachmentDescriptor {
                attachment: out_view,
                resolve_target: None,
                load_op: LoadOp::Clear,
                store_op: StoreOp::Store,
                clear_color: Color::BLACK,
            }],
            depth_stencil_attachment: None,
        });

        render_pass.set_bind_group(0, bind_group, &[]);
        render_pass.set_pipeline(pipeline);
        render_pass.draw(0..3, 0..1);
    }

    encoder.copy_texture_to_buffer(
        TextureCopyView {
            texture: texture,
            mip_level: 0,
            array_layer: 0,
            origin: Origin3d::ZERO,
        },
        BufferCopyView {
            buffer: buffer,
            offset: 0,
            bytes_per_row: TEXTURE_SIZE.0 * 4,
            rows_per_image: TEXTURE_SIZE.1,
        },
        Extent3d {
            width: TEXTURE_SIZE.0,
            height: TEXTURE_SIZE.1,
            depth: 1,
        },
    );

    let cmd_buffer = encoder.finish();
    queue.submit(&[cmd_buffer]);
}

async fn run() {
    let adapter = get_adapter().await;
    let (device, queue) = get_device(&adapter).await;
    let (pipeline, bind_group) = create_pipeline(&device);
    let (texture, view) = create_output_texture(&device);
    let buffer = create_output_buffer(&device);

    render_frame(
        &device,
        &queue,
        &view,
        &pipeline,
        &bind_group,
        &texture,
        &buffer,
    );

    let buffer_mapping = buffer.map_read(0, TEXTURE_SIZE.0 as u64 * TEXTURE_SIZE.1 as u64 * 4);

    device.poll(Maintain::Wait);

    if let Ok(buffer_mapping) = buffer_mapping.await {
        let mut encoder = png::Encoder::new(
            File::create("output.png").unwrap(),
            TEXTURE_SIZE.0,
            TEXTURE_SIZE.1,
        );
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(buffer_mapping.as_slice()).unwrap();
    }
}

fn main() {
    futures::executor::block_on(run());
}
