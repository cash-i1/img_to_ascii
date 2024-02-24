use std::usize;
use std::fs::File;
use std::io::prelude::*;
use image::{io::Reader, GenericImageView, Rgba};

fn max(rgba: Rgba<u8>) -> u8 {
    let mut m = 0; 
    for i in 0 as u8..=3 as u8 {
        // println!("{}", rgba[i as usize]);
        if rgba[i as usize] > m { 
            m = rgba[i as usize];
        }
    }
    m
}

fn average(rgba: Rgba<u8>) -> i32 {
    let mut sum: i32 = 0;
    let mut len: i32 = 0;
    for i in 0..=3 {
        sum += rgba[i] as i32;
        len += 1;
    }
    sum / len
}

fn main() {
    let img = Reader::open("src/apple.png").unwrap().decode().unwrap();
    let dim: (u32, u32) = img.dimensions();
    let chars: Vec<char> = r"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,".chars().rev().collect();
    
    let mut ascii_img: Vec<Vec<char>> = vec![vec![','; img.dimensions().0 as usize]; img.dimensions().1 as usize];

    for pixel in img.pixels() {
        // println!("{:?} ;; pos: x={}, y={} ;; thing: {:?} ;; max: {}", pixel, pixel.0, pixel.1, pixel.2, max(pixel.2));
        // let brightness: f32 = ((max(pixel.2) as f32 - 43.) / 255.).round();
        

        let brightness: f32 = (((average(pixel.2) as f32) / 255.) * 100.0).round() / 100.0;
        let c: char = chars[(chars.len() as f32 * brightness) as usize - 1];
        ascii_img[pixel.1 as usize][pixel.0 as usize] = c;
        // println!("{}", brightness)
        // println!("{}, '{}', {}", brightness, c, chars.len());
        



    }
   
    let mut ascii_str = String::new();

    for (idx, row) in ascii_img.iter().enumerate() {
        for char in row {
            if idx % 2 == 0 {
                print!("{}", char);
                ascii_str += char.to_string().as_str();
            }
        }
        if idx % 2 == 0 {
            println!("");
            ascii_str += "\n";

        }
    }

    let mut file = File::create("./src/file/crap.txt").expect("pooing ansd");
    file.write_all(ascii_str.as_bytes());


}















