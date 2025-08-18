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

let poligono2 = [
    (321, 335),
    (288, 286),
    (339, 251),
    (374, 302),
];

let poligono3 = [
    (377, 249),
    (411, 197),
    (436, 249),
];

let poligono4 = [
    (413, 177),
    (448, 159),
    (502, 88),
    (553, 53),
    (535, 36),
    (676, 37),
    (660, 52),
    (750, 145),
    (761, 179),
    (672, 192),
    (659, 214),
    (615, 214),
    (632, 230),
    (580, 230),
    (597, 215),
    (552, 214),
    (517, 144),
    (466, 180),
];

let poligono5 = [
    (682, 175),
    (708, 120),
    (735, 148),
    (739, 170),
];


framebuffer.set_current_color(Color::RED);
fill_polygon(&mut framebuffer, &poligono1);

framebuffer.set_current_color(Color::BLUE);
fill_polygon(&mut framebuffer, &poligono2);

framebuffer.set_current_color(Color::GREEN);
fill_polygon(&mut framebuffer, &poligono3);

framebuffer.set_current_color(Color::YELLOW);
fill_polygon(&mut framebuffer, &poligono4);

framebuffer.set_current_color(Color::PURPLE);
fill_polygon(&mut framebuffer, &poligono5);


    while !rl.window_should_close() {
        framebuffer.swap_buffers(&mut rl, &thread);
    }
}

