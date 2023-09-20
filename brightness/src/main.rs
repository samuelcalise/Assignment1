use csc411_image::{Read, GrayImage};
use std::env;
use std::io;
use std::io::BufRead;

fn main() {
    //let mut input = env::args().nth(1);
    let args: Vec<String> = env::args().collect();
    // let string: String = String::new();
    // string = input;

    // match input {
    //     Ok(value) => {
    //         assert!(env::args().len() <= 2);
    //     }
    //     Err(error) => {

    //     }
    // }
    match args.len() {
        1 => { //stanard input
            let mut input = String::new();
            io::stdin().read_line(& mut input);
            let img = GrayImage::read(&str input);
        }
    }

    

    let img = GrayImage::read(input.as_deref()).unwrap();

    let denom = img.denominator as f32;
    let mut total: f32 = 0.0;

    for pixel in &img.pixels{
        total += pixel.value as f32 / denom;
    }

    let average = total / img.pixels.len() as f32;

    println!("{:.3}", average);
}