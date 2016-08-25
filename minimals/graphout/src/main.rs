extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate graphics;

use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::Events;
use piston::input::RenderEvent;
use graphics::rectangle;

const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

fn main() {
    let mut window: Window =
        WindowSettings::new("Hello World!", [512; 2])
            .build().unwrap();

    let opengl = OpenGL::V3_2;
    let mut gl = GlGraphics::new(opengl);

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, gl| {
                let square = rectangle::square(0.0, 0.0, 50.0);
                rectangle(RED, square, c.transform, gl);
            });
        }
    }
}
