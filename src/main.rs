mod gen;

use macroquad::prelude::*;

#[macroquad::main("main")]
async fn main() {
    let ascii = gen::generate_ascii("src/images/bad_apple.png".to_string(), false);
    let text_size = 1.; 

    loop {
        clear_background(BLACK);

        for (idx, line) in ascii.lines().enumerate() {
            draw_text(line, 1., idx as f32 * text_size, text_size, WHITE); 
        }

        draw_text(get_fps().to_string().as_str(), 10., 10., 20., RED); 
        next_frame().await
    }
    

}

