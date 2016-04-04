extern crate sdl2;
extern crate byteorder;

mod resources;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::BlendMode;

use resources::{Resources};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Ultima Online RS", 1280, 800)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    renderer.set_blend_mode(BlendMode::Blend);
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();

    let mut resources = Resources::new();
    
    for screen_y in 0..10 {
        for screen_x in 0..10 {
            //let map = resources.get_map_block(199 + screen_x, 210 + screen_y); // Britain
            let map = resources.get_map_block(182 + screen_x, 106 + screen_y);

            for (i, cell) in map.cells.iter().enumerate() {
                let y = i as i32 / 8;
                let x = i as i32 % 8;
                let loc_y = y * 44 + screen_y * 176;
                let loc_x = x * 44 + screen_x * 176;
                let mut iso_y = (loc_x + loc_y) / 2;
                let mut iso_x = (loc_x - loc_y) / 2;
                iso_y = iso_y - 500 + (cell.z as i32 * 4);
                iso_x = iso_x + 1000;

                let tile = resources.get_land(&renderer, cell.tile_id as usize);
                renderer.copy(tile, None, Some(Rect::new(iso_x, iso_y, 44, 44)));
            }
        }
    }

    /*
    let map = resources.get_map_block(199, 210);
    for (i, cell) in map.cells.iter().enumerate() {
        let y = i as i32 / 8;
        let x = i as i32 % 8;
        let loc_y = y * 44;
        let loc_x = x * 44;
        let iso_y = (loc_x + loc_y) / 2;
        let iso_x = (loc_x - loc_y) / 2;

        let tile = resources.get_land(&renderer, cell.tile_id as usize);
        renderer.copy(tile, None, Some(Rect::new(iso_x, iso_y, 44, 44)));
    }

    let map = resources.get_map_block(199, 209);
    for (i, cell) in map.cells.iter().enumerate() {
        let y = i as i32 / 8;
        let x = i as i32 % 8;
        let loc_y = y * 44;
        let loc_x = x * 44 + 176;
        let iso_y = (loc_x + loc_y) / 2;
        let iso_x = (loc_x - loc_y) / 2;

        let tile = resources.get_land(&renderer, cell.tile_id as usize);
        renderer.copy(tile, None, Some(Rect::new(iso_x, iso_y, 44, 44)));
    }
    */

    renderer.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
    }
}
