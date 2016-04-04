use std::io::prelude::*;
use std::fs::File;
use std::io::Cursor;
use std::io::SeekFrom;
use byteorder::{LittleEndian, ReadBytesExt};

pub struct FileIndex {
    pub lookup: i32,
    pub size: i32,
    pub extra: i32,
}

pub fn get_index(path: &str, id: u64) -> FileIndex {
    let mut index_file = File::open(path).unwrap();
    index_file.seek(SeekFrom::Start(id * 12 as u64)).unwrap();

    let mut file_buf = vec![0; 12];
    index_file.read_exact(&mut file_buf).unwrap();
    let mut buf = Cursor::new(file_buf);

    FileIndex {
        lookup: buf.read_i32::<LittleEndian>().unwrap(),
        size: buf.read_i32::<LittleEndian>().unwrap(),
        extra: buf.read_i32::<LittleEndian>().unwrap(),
    }
}
