use pixels::{Pixels, SurfaceTexture};
use pixels_u32::PixelsExt;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Pixels-u32 demo")
        .with_inner_size(PhysicalSize {
            width: 800,
            height: 600,
        })
        .build(&event_loop)?;
    let mut pixels = {
        let size = window.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, &window);
        Pixels::new(size.width, size.height, surface)?
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    resize(&mut pixels, size.width, size.height);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    resize(&mut pixels, new_inner_size.width, new_inner_size.height);
                }
                _ => {}
            },
            Event::MainEventsCleared => {}
            Event::RedrawRequested(_) => {
                draw(&mut pixels);
            }
            _ => {}
        }
    });
}

fn resize(pixels: &mut Pixels, width: u32, height: u32) {
    pixels.resize_surface(width, height);
    pixels.resize_buffer(width, height);
}

fn draw(pixels: &mut Pixels) {
    let frame = pixels.get_frame_u32();

    frame.iter_mut().for_each(|pixel| {
        *pixel = 0xff302010;
    });

    pixels.render().unwrap();
}
