use piston_window::{EventLoop, PistonWindow, RenderEvent, WindowSettings};

fn main() {
    let mut window: PistonWindow = WindowSettings::new("naanes", [10, 10])
        .exit_on_esc(true)
        .build()
        .unwrap();

    window.set_max_fps(60);

    let mut fps = fps_counter::FPSCounter::default();
    loop {
        if let Some(event) = window.next() {
            if let Some(_) = event.render_args() {
                println!("fps: {:}", fps.tick());
            }
        }
    }
}
