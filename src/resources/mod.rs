use sdl2::render::{Texture, Renderer};
use std::collections::HashMap;

pub mod file_index;
pub mod art;
pub mod tex_maps;
pub mod maps;

use resources::art::read_land;
//use resources::file_index::{read_index, FileIndex};
use resources::maps::{MapBlock, read_block};

pub struct Resources {
    //art_index: Vec<FileIndex>,
    land_textures: HashMap<usize, Texture>,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            //art_index: read_index("tmp/artidx.mul"),
            land_textures: HashMap::new(),
        }
    }

    pub fn get_land(&mut self, renderer: &Renderer, index: usize) -> &Texture {
        self.land_textures.entry(index).or_insert_with(|| read_land(renderer, index))
    }

    pub fn get_map_block(&self, x: i32, y: i32) -> MapBlock {
        read_block(x, y)
    }
}
