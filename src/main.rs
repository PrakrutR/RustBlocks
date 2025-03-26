use macroquad::prelude::*;

#[macroquad::main("RustBlocks")]
async fn main() {
    // Initialize logger
    // This would typically use env_logger, but we're keeping it simple for the initial test
    
    // Game loop
    loop {
        // Process frame
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