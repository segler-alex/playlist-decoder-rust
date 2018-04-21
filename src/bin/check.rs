extern crate playlist_decoder;

use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    match env::args().nth(1) {
        Some(filename) => {
            let mut f = File::open(filename).expect("file not found");
            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .expect("something went wrong reading the file");
            let list = playlist_decoder::decode(&contents);
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
