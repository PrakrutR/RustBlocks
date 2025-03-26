#[cfg(target_os = "android")]
use ndk_glue;

#[cfg(target_os = "android")]
#[no_mangle]
#[ndk_glue::main(name = "android_main")]
pub fn android_main() {
    main();
}

use macroquad::prelude::*;

#[macroquad::main("RustBlocks")]
pub async fn main() {
    loop {
        clear_background(BLACK);
        
        // Game rendering code
        let text = "RustBlocks - Tetris in Rust";
        let font_size = 30.0;
        let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
        
        let (screen_width, screen_height) = (screen_width(), screen_height());
        let text_x = screen_width / 2.0 - text_dimensions.width / 2.0;
        let text_y = screen_height / 2.0;
        
        draw_text_ex(
            text,
            text_x,
            text_y,
            TextParams {
                font_size: font_size as u16,
                color: WHITE,
                ..Default::default()
            },
        );
        
        draw_rectangle(
            screen_width / 2.0 - 50.0,
            screen_height / 2.0 + 50.0,
            100.0,
            100.0,
            RED,
        );
        
        next_frame().await;
    }
}