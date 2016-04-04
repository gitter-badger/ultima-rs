/*
let mut art_index = Vec::new();

// Read Index
//let mut file_idx = File::open("tmp/artidx.mul").unwrap();
let mut file_idx = File::open("tmp/texidx.mul").unwrap();
let file_length = file_idx.metadata().unwrap().len();
let nr_of_art = file_length / 12;

let mut file_buf = Vec::new();
file_idx.read_to_end(&mut file_buf);
let mut buf = Cursor::new(file_buf);

for idx in 0..nr_of_art {
    art_index.push(FileIndex {
        lookup: buf.read_i32::<LittleEndian>().unwrap(),
        size: buf.read_i32::<LittleEndian>().unwrap(),
        extra: buf.read_i32::<LittleEndian>().unwrap(),
    });
}

let land = &art_index[750];

let mut file = File::open("tmp/texmaps.mul").unwrap();

file.seek(SeekFrom::Start(land.lookup as u64));
let mut land_buf = vec![0; land.size as usize];
file.read_exact(&mut land_buf);

// Flag
let mut land_cursor = Cursor::new(land_buf.to_vec());

let width = if land.extra == 0 {
    64
}
else {
    128
};

let mut texture = renderer.create_texture_streaming(PixelFormatEnum::ARGB1555, width, width).unwrap();
let mut i = 0;
let mut colors_offset = 0;
texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
    for i in 0..land.size as usize {
        buffer[i] = land_buf[i];
    }
    /*
       let mut x = 22;
       let mut y = 0;
       let mut line_width = 2;

       for g in 0..2 {
       x -= 1;
       for h in 0..line_width {
       let offset = y * pitch + x * 2;
       let mut color_1 = [0; 1];
       file.read_exact(&mut color_1).unwrap();
       buffer[offset] = color_1[0];
       let mut color_2 = [0; 1];
       file.read_exact(&mut color_2).unwrap();
       buffer[offset + 1] = color_2[0];

       println!("Color: {},{}", color_1[0], color_2[0]);
/*
let col_offset = colors_offset + h;
buffer[offset] = colors[col_offset];
buffer[offset + 1] = colors[col_offset + 1];
colors_offset += 2;
*/
}
y += 1;
line_width += 2;
}

for g in 0..2 {
    for h in 0..line_width {
        let offset = y * pitch + x * 2;
        let mut colors = [0; 2];
        file.read_exact(&mut colors);
        buffer[offset] = colors[0];
        buffer[offset + 1] = colors[1];
        /*
           let col_offset = colors_offset + h;
           buffer[offset] = colors[col_offset];
           buffer[offset + 1] = colors[col_offset + 1];
           colors_offset += 2;
           */
    }
    x += 1;
    y += 1;
    line_width -= 2;
}
*/
}).unwrap();
*/
