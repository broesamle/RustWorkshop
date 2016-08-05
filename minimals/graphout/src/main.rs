extern crate piston;
extern crate glutin_window;

use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;

fn main() {
    let window: Window =
        WindowSettings::new("Hello World!", [512; 2])
            .build().unwrap();
}
