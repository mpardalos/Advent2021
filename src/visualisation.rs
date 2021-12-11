use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

pub trait WindowApp {
    const WINDOW_NAME: &'static str;
    const WINDOW_WIDTH: u32;
    const WINDOW_HEIGHT: u32;
    const WINDOW_FPS: Option<u32>;

    fn draw_frame(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String>;
    fn handle_event(&mut self, _event: Event) {}
    fn reset(&mut self) {}

    fn run_window(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(Self::WINDOW_NAME, Self::WINDOW_WIDTH, Self::WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape | Keycode::Q),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(Keycode::R),
                        ..
                    } => self.reset(),
                    _ => self.handle_event(event),
                }
            }

            self.draw_frame(&mut canvas).unwrap();

            if let Some(fps) = Self::WINDOW_FPS {
                std::thread::sleep(Duration::new(0, 1_000_000_000u32 / fps));
            }
        }
    }
}
