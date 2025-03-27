// src/lib.rs
use macroquad::prelude::*;

pub async fn run_game() {
    // Your game loop remains unchanged
    loop {
        clear_background(BLACK);
        
        // Draw some text to verify everything is working
        let text = "RustBlocks - Tetris in Rust";
        let font_size = 30.0;
        let text_size = measure_text(text, None, font_size as u16, 1.0);
        
        // Center the text on screen
        let screen_width = screen_width();
        let screen_height = screen_height();
        let text_x = screen_width / 2.0 - text_size.width / 2.0;
        let text_y = screen_height / 2.0;
        
        // Draw the text
        draw_text(text, text_x, text_y, font_size, WHITE);
        
        // Draw a simple rectangle
        draw_rectangle(
            screen_width / 2.0 - 50.0,
            screen_height / 2.0 + 50.0,
            100.0,
            100.0,
            RED,
        );
        
        // Wait for next frame
        next_frame().await;
    }
}

// The macroquad_main macro will handle the platform-specific initialization
// This is a helper function that is common across platforms
pub fn game_main() {
    // The window configuration
    let conf = Conf {
        window_title: "RustBlocks".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    };
    
    // Start macroquad with our configuration and game loop
    macroquad::Window::from_config(conf, async {
        run_game().await;
    });
}   

// This is needed for Android
#[cfg(target_os = "android")]
#[no_mangle]
#[ndk_glue::main]
pub fn android_main() {
    // Initialize logging
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::Level::Debug)
    );
    
    log::info!("Starting RustBlocks");
    
    let conf = Conf {
        window_title: "RustBlocks".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    };
    
    macroquad::Window::from_config(conf, async {
        if let Err(e) = run_game().await {
            log::error!("Game crashed: {:?}", e);
        }
    });
}