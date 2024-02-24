use std::usize;

use image::{io::Reader, GenericImageView, Rgba};

fn max(rgba: Rgba<u8>) -> u8 {
    let mut m = 0; 
    for i in 0 as u8..=3 as u8 {
        if rgba[i as usize] > m { 
            m = rgba[i as usize];
        }
    }
    m
}

fn main() {
    let img = Reader::open("src/rgbb.png").unwrap().decode().unwrap();
    let dim: (u32, u32) = img.dimensions();
    let chars: Vec<char> = r"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,".chars().collect();
    
    let mut ascii_img: Vec<Vec<char>> = vec![];

    
    for pixel in img.pixels() {
        // println!("{:?} ;; pos: x={}, y={} ;; thing: {:?} ;; max: {}", pixel, pixel.0, pixel.1, pixel.2, max(pixel.2));
        // let brightness: f32 = ((max(pixel.2) as f32 - 43.) / 255.).round();
        let brightness: f32 = (((max(pixel.2) as f32) / 255.) * 100.0).round() / 100.0;
        let c: char = chars[(chars.len() as f32 * brightness) as usize - 1];
        
        println!("{}, '{}', {}", brightness, c, chars.len());


    }
}















