mod framebuffer;
mod polygon_fill;

use framebuffer::Framebuffer;
use polygon_fill::fill_polygon;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Polígono Relleno")
        .build();

    let mut framebuffer = Framebuffer::new(800, 600, Color::BLACK);
    framebuffer.set_current_color(Color::RED);

    let poligono1 = [(165, 380), (185, 360), (180, 330), (207, 345), (233, 330), (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),];


framebuffer.set_current_color(Color::RED);
fill_polygon(&mut framebuffer, &poligono1);



    while !rl.window_should_close() {
        framebuffer.swap_buffers(&mut rl, &thread);
    }
}

