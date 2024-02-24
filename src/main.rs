mod gen;

use macroquad::prelude::*;

#[macroquad::main("main")]
async fn main() {
    let ascii = gen::generate_ascii("src/images/red_apple.png".to_string(), false);
    let text_size = 2.; 

    loop {
        clear_background(BLACK);

        for (idx, line) in ascii.lines().enumerate() {
            draw_text(line, 1., idx as f32 * text_size, text_size, WHITE); 
        }

    
        next_frame().await
    }
    

}

