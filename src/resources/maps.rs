use std::io::prelude::*;
use std::fs::File;
use std::io::Cursor;
use std::io::SeekFrom;
use byteorder::{LittleEndian, ReadBytesExt};

pub struct MapCell {
    pub tile_id: i16,
    pub z: i8,
}

pub struct MapBlock {
    pub header: i32,
    pub cells: Vec<MapCell>,
}

pub fn read_block(x: i32, y: i32) -> MapBlock {
    let mut file = File::open("tmp/map0.mul").unwrap();

    let lookup = ((x * 512) + y) * 196;

    file.seek(SeekFrom::Start(lookup as u64)).unwrap();
    let mut map_buf = vec![0; 196];
    file.read_exact(&mut map_buf).unwrap();
    let mut map_cursor = Cursor::new(map_buf);

    let header = map_cursor.read_i32::<LittleEndian>().unwrap();

    let mut cells = Vec::new();

    for _ in 0..64 {
        cells.push(MapCell{
            tile_id: map_cursor.read_i16::<LittleEndian>().unwrap(),
            z: map_cursor.read_i8().unwrap(),
        });
    }

    MapBlock {
        header: header,
        cells: cells,
    }
}
