use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
};
use silversnail::{glow, GlVersion, Renderer};

use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};

fn main() {
    SimpleLogger::init(
        LevelFilter::Trace,
        ConfigBuilder::new()
            .set_time_level(LevelFilter::Off)
            .set_location_level(LevelFilter::Off)
            .set_target_level(LevelFilter::Off)
            .set_thread_level(LevelFilter::Off)
            .build(),
    )
    .unwrap();

    let el = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_title("silversnail native")
        .with_inner_size((640, 480).into());

    let windowed_context = unsafe {
        ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(wb, &el)
            .unwrap()
            .make_current()
            .unwrap()
    };

    let mut renderer = Renderer::new(
        glow::Context::from_loader_function(|s| windowed_context.get_proc_address(s) as *const _),
        GlVersion::Core410,
    );

    el.run(move |e, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match e {
            Event::EventsCleared => {
                windowed_context.window().request_redraw();
            }

            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::RedrawRequested => {
                    renderer.render_frame(640, 480);
                    windowed_context.swap_buffers().unwrap();
                }

                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });
}
