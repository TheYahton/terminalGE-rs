use std::num::NonZeroU32;
use std::rc::Rc;
use terminalge_rs::drawing::{self, Color, Display};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

struct PixelBuffer<'a> {
    screen: softbuffer::Buffer<'a, Rc<winit::window::Window>, Rc<winit::window::Window>>,
    height: i64,
    width: i64,
}

impl Display for PixelBuffer<'_> {
    fn plot(&mut self, x: i64, y: i64, color: &drawing::Color) {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return;
        }
        self.screen[(x + y * self.width) as usize] =
            color.2 as u32 | ((color.1 as u32) << 8) | ((color.0 as u32) << 16);
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(WindowBuilder::new().build(&event_loop).unwrap());
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    event_loop
        .run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Wait);

            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::RedrawRequested,
                } if window_id == window.id() => {
                    let (width, height) = {
                        let size = window.inner_size();
                        (size.width, size.height)
                    };
                    surface
                        .resize(
                            NonZeroU32::new(width).unwrap(),
                            NonZeroU32::new(height).unwrap(),
                        )
                        .unwrap();

                    let mut buffer = PixelBuffer {
                        screen: surface.buffer_mut().unwrap(),
                        height: height as i64,
                        width: width as i64,
                    };

                    drawing::circle(&mut buffer, 200, 200, 150, &Color(255, 255, 0));

                    buffer.screen.present().unwrap();
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => {
                    elwt.exit();
                }
                _ => {}
            }
        })
        .unwrap();
}
