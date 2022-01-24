use std::fs::File;
use std::io;

use fastnbt::de::from_bytes;
use fastnbt::stream::{Parser, Value};

use fastanvil::{Chunk, JavaChunk, RegionBuffer};

pub fn read_level(lvl_name: &str) {
    let path = ["./src/level/lvls/", lvl_name, "/region/r.0.0.mca"].concat();
    let file = File::open(path)
        .expect(&format!("Can't open file {}", lvl_name));

    let region = RegionBuffer::new(file);
    let data = region.load_chunk(0, 0 ).unwrap();

    let chunk: JavaChunk = from_bytes(data.as_slice()).unwrap();

    println!("chunk {:?}", chunk);
    println!("biome {:?}", chunk.biome(0,0, 0));
    println!("block {:?}", chunk.block(0,-61, 0));
}