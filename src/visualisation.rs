use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::{event::Event, render::WindowCanvas};
use std::time::{Duration, Instant};

pub fn show_text(
    canvas: &mut WindowCanvas,
    font: &Font,
    x: i32,
    y: i32,
    text: &str,
) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    let surface = font
        .render(text)
        .blended(Color::WHITE)
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    let TextureQuery { width, height, .. } = texture.query();
    canvas.copy(&texture, None, Some(Rect::new(x, y, width, height)))?;
    Ok(())
}

pub trait WindowApp {
    const WINDOW_NAME: &'static str;
    const WINDOW_WIDTH: u32;
    const WINDOW_HEIGHT: u32;
    const WINDOW_FPS: Option<u32>;
    const SHOW_FPS: bool = true;

    fn draw_frame(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String>;
    fn handle_event(&mut self, _event: Event) {}
    fn reset(&mut self) {}

    fn run_window(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let window = video_subsystem
            .window(Self::WINDOW_NAME, Self::WINDOW_WIDTH, Self::WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

        // Load a font
        let mut font = ttf_context.load_font("/usr/share/fonts/dejavu-sans-fonts/DejaVuSans.ttf", 18).unwrap();
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let mut target_fps = Self::WINDOW_FPS;
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut frame_time_counter = Instant::now();

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
                    Event::KeyDown {
                        keycode: Some(Keycode::O),
                        ..
                    } => {
                        target_fps = target_fps
                            .map(|f| if f > 1 { f - 1 } else { f })
                            .or(Some(60))
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::P),
                        ..
                    } => target_fps = target_fps.map(|f| f.saturating_add(1)).or(Some(60)),
                    _ => self.handle_event(event),
                }
            }

            self.draw_frame(&mut canvas).unwrap();

            if let Some(fps) = target_fps {
                std::thread::sleep(Duration::new(0, 1_000_000_000u32 / fps));
            }

            let now = Instant::now();
            let frametime = now - frame_time_counter;
            frame_time_counter = now;
            show_text(
                &mut canvas,
                &font,
                0,
                0,
                &format!("{:.0}", 1. / frametime.as_secs_f32()),
            )
            .unwrap();
            canvas.present();
        }
    }
}
