use image::{io::Reader, GenericImageView, Rgba};

fn max(rgba: Rgba<u8>) -> u8 {
    let mut m = 0; 
    for i in 0 as u8..=3 as u8 {
        println!("{i}: {}", rgba[i as usize]);
        if rgba[i as usize] > m { 
            m = rgba[i as usize];
            println!("IS: {m} == {i}");
        }
    }
    m
}

fn main() {
    let img = Reader::open("src/rgbb.png").unwrap().decode().unwrap();
    
    for pixel in img.pixels() {
        println!("{:?} ;; pos: x={}, y={} ;; thing: {:?} ;; max: {}", pixel, pixel.0, pixel.1, pixel.2, max(pixel.2));
    }
}
