use std::fs::File;
use std::io::prelude::*;
use image::{io::Reader, GenericImageView, Rgba};

fn average(rgba: Rgba<u8>) -> i32 {
    let mut sum: i32 = 0;
    let mut len: i32 = 0;
    for i in 0..=3 {
        sum += rgba[i] as i32;
        len += 1;
    }
    sum / len
}

pub fn generate_ascii(path: String, print: bool) -> String {
    let img = Reader::open(path).unwrap().decode().unwrap();
    let chars: Vec<char> = r"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,".chars().rev().collect(); 
    let mut ascii_img: Vec<Vec<char>> = vec![vec![','; img.dimensions().0 as usize]; img.dimensions().1 as usize];
    for pixel in img.pixels() {
        let brightness: f32 = (((average(pixel.2) as f32) / 255.) * 100.0).round() / 100.0;
        let c: char = chars[(chars.len() as f32 * brightness) as usize - 1];

        ascii_img[pixel.1 as usize][pixel.0 as usize] = c;
    }
   
    let mut ascii_str = String::new();

    for (idx, row) in ascii_img.iter().enumerate() {
        for char in row {
            if idx % 2 == 0 {
                if print {
                    print!("{}", char);

                }
                ascii_str += char.to_string().as_str();
            }
        }
        if idx % 2 == 0 {
            if print {
                println!("");

            }
            ascii_str += "\n";

        }
    }

    // let mut file = File::create("./src/file/crap.txt").expect("pooing ansd");
    // file.write_all(ascii_str.as_bytes());
    //
    ascii_str

}
