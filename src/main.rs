#![allow(non_snake_case)]
#[macro_use]
extern crate glium;
extern crate image;
use glium::glutin::*;
use glium::glutin::ControlFlow::*;
use std::io::Cursor;
use std::*;



#[path = "tuto-07-teapot.rs"]
pub mod teapot;
mod gui;
mod render;
mod util;
mod camera;
use camera::*;
use util::*;



pub fn main() 
{
    // test_rotates();
    

    println!("{}", std::env::current_dir().unwrap().display());
    let image = image::load(Cursor::new(&include_bytes!("../img/download.png")[..]),
                        image::PNG).expect("Failed to load image").to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_transparency(true)
        .with_dimensions(1024, 768)
        .with_title("Hello world");
    let context = glium::glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    
    let texture:glium::texture::Texture2d = glium::texture::Texture2d::new(&display, image).unwrap();

    let mut closed = false;

    let player_position = [0.0, 0.0, 0.0f32];
    let player_direction = [0.0, 0.0, 1.0];
    let player_up = [0.0, 1.0, 0.0];
    let mut player = camera::Camera{pos: camera::Position{vec: player_position}, dir: camera::Direction{forward: player_direction, up: player_up}};
    let mut last_touch = [0.0, 0.0];
    while !closed {

        render::draw(&display, &texture, &player);
        events_loop.poll_events(|e| handle_event(e, &mut closed, &mut player, &mut last_touch));
        // print!("{:?}", player_position);
    }
     thread::sleep(time::Duration::from_millis(1));
    
}

static rotation_const: f32 = 0.1;

fn handle_event(e: Event, breaker: &mut bool, 
                player: &mut Camera,
                last_touch: &mut[f32; 2]) 
{
    
    match gui::handle_event(e, last_touch)
    {
        gui::EventResult::Flow(Break) => *breaker = true,
        gui::EventResult::Direction(direction) => 
        {
            match direction
            {
                gui::Direction::UP => 
                {
                    player.pos.vec[1] += 0.05f32;
                },
                gui::Direction::DOWN =>
                {
                    player.pos.vec[1] -= 0.05f32;
                },
                gui::Direction::LEFT =>
                {
                    player.pos.vec[2] -= 0.05f32;
                },
                gui::Direction::RIGHT =>
                {
                    player.pos.vec[2] += 0.05f32;
                },
                _ => ()
            }
        }, 
        gui::EventResult::Rotate(r) =>
        {
            // println!("{:?}", player.dir);
            // println!("{}", dot_product(player.dir.up, player.dir.forward));
            player.dir.rotate_x(rotation_const * r.x);
            player.dir.rotate_y(rotation_const * r.y);
            player.dir.rotate_z(rotation_const * r.z * 10.0);
            player.dir.normalize();
        },
        _ => ()
    }
}


