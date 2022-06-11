use piston_window::*;
use std::path::*;

const WIDTH: f64 = 1000.0;
const HEIGHT: f64 = 1000.0;

const FPS: f64 = 144.0;

const BG_COLOR: &str = "000000";

fn main() {
    let mut window: PistonWindow = WindowSettings::new("DnD Combat Simulator", [WIDTH, HEIGHT])
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap();
        
    let mut timer: f64 = 0.0;
    let mut events = window.events;

    let mut glyphs = window.load_font(Path::new("assets").join("OldSchoolAdventures.ttf")).unwrap();

    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g, device| {
                clear(color::hex(BG_COLOR), g);

                text::Text::new_color(color::hex("ffffff"), 12).draw(
                    "Hello World!",
                    &mut glyphs,
                    &DrawState::default(),
                    c.transform.trans(10.0, 22.0),
                    g
                ).unwrap();

                glyphs.factory.encoder.flush(device);
            });

        }
        if let Some(u) = e.update_args() {
            if timer >= 1.0 / FPS{
                timer = u.dt;
            } else {
                timer += u.dt;
            }
        }
    }
}