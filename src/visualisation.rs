use minifb::{Window, WindowOptions};
use raqote::DrawTarget;

pub fn run_window<F: FnMut(&Window, &mut DrawTarget) -> bool>(
    name: &str,
    width: usize,
    height: usize,
    draw: &mut F,
) {
    let mut window: Window = Window::new(
        name,
        width,
        height,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    let mut size = window.get_size();
    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);
    while window.is_open() {
        if window.get_size() != size {
            size = window.get_size();
            dt = DrawTarget::new(size.0 as i32, size.1 as i32);
        }

        if draw(&window, &mut dt) {
            window
                .update_with_buffer(dt.get_data(), size.0, size.1)
                .unwrap();
        }
    }
}
