use std::io::prelude::*;
use std::fs::File;
use std::io::Cursor;
use std::io::SeekFrom;
use byteorder::ReadBytesExt;
use sdl2::render::{Texture, Renderer, BlendMode};
use sdl2::pixels::PixelFormatEnum;
use resources::file_index::get_index;

pub fn read_land(renderer: &Renderer, id: usize) -> Texture {
    let land = get_index("tmp/artidx.mul", id as u64);

    let mut file = File::open("tmp/art.mul").unwrap();

    file.seek(SeekFrom::Start(land.lookup as u64)).unwrap();
    let mut land_buf = vec![0; land.size as usize];
    file.read_exact(&mut land_buf).unwrap();
    let mut land_cursor = Cursor::new(land_buf);

    let mut texture = renderer.create_texture_streaming(PixelFormatEnum::ARGB1555, 44, 44).unwrap();
    texture.set_blend_mode(BlendMode::Blend);
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        let mut x = 22;
        let mut y = 0;
        let mut line_width = 0;

        for _ in 0..22 {
            x -= 1;
            line_width += 2;
            for i in 0..line_width {
                let offset = (y * pitch) + (x * 2) + (i * 2);
                let color_1 = land_cursor.read_u8().unwrap();
                let mut color_2 = land_cursor.read_u8().unwrap();
                color_2 = color_2 | (1 << 7);
                buffer[offset] = color_1;
                buffer[offset + 1] = color_2;
            }
            y += 1;
        }

        for _ in 0..22 {
            for i in 0..line_width {
                let offset = (y * pitch) + (x * 2) + (i * 2);
                let color_1 = land_cursor.read_u8().unwrap();
                let mut color_2 = land_cursor.read_u8().unwrap();
                color_2 = color_2 | (1 << 7);
                buffer[offset] = color_1;
                buffer[offset + 1] = color_2;
            }
            x += 1;
            y += 1;
            line_width -= 2;
        }
    }).unwrap();

    texture
}
