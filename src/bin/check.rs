extern crate playlist_decoder;

use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    match env::args().nth(1) {
        Some(filename) => {
            let mut f = File::open(filename).expect("file not found");
            let mut contents = String::new();
            let bytes: Vec<u8> = f.bytes().map(|x| x.expect("Byte read error")).collect();
            let out = String::from_utf8_lossy(&bytes);
            let content = out.to_string();
            let list = playlist_decoder::decode(&content);
            for item in list {
                println!("{:?}", item);
            }
            println!("is_content_hls: {}", playlist_decoder::is_content_hls(&contents));
        }
        None => {
            println!("Call with 1 parameter");
        }
    }
}
