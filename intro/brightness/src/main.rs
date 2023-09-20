use csc411_image::{Read, GrayImage};
use std::env;

fn main() {
    let input = env::args().nth(1);
    let img = GrayImage::read(input.as_deref()).unwrap();
    let denom = img.denominator as f32;
    let mut total: f32 = 0.0;

    for pixel in &img.pixels{
        total += pixel.value as f32 / denom;
    }

    let average = total / img.pixels.len() as f32;

    println!("{:.3}", average);
}