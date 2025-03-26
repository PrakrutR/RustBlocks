// This file is required for Android builds
// It exports the necessary components for the Android library

// Re-export any modules that need to be accessible from the Android side
// For now, this is minimal, but we'll expand it as the project grows

#[cfg(target_os = "android")]
use ndk_glue;

// This macro generates the required JNI glue code for Android
#[cfg(target_os = "android")]
#[no_mangle]
#[ndk_glue::main(name = "android_main")]
pub fn android_main() {
    // This calls our regular main function
    main();
}

// Regular main function that matches what's in main.rs
fn main() {
    // This will be called from android_main() on Android
    // or directly on other platforms
    macroquad::start(
        macroquad::conf::Conf {
            window_title: "RustBlocks".to_string(),
            window_width: 800,
            window_height: 600,
            ..Default::default()
        },
        async {
            use macroquad::prelude::*;
            
            loop {
                clear_background(BLACK);
                
                // Draw some text
                let text = "RustBlocks - Tetris in Rust";
                let font_size = 30.0;
                let text_size = measure_text(text, None, font_size as u16, 1.0);
                
                let screen_width = screen_width();
                let screen_height = screen_height();
                let text_x = screen_width / 2.0 - text_size.width / 2.0;
                let text_y = screen_height / 2.0;
                
                draw_text(text, text_x, text_y, font_size, WHITE);
                
                // Draw a simple rectangle
                draw_rectangle(
                    screen_width / 2.0 - 50.0,
                    screen_height / 2.0 + 50.0,
                    100.0,
                    100.0,
                    RED,
                );
                
                next_frame().await;
            }
        },
    );
}