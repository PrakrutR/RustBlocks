use rustblocks_lib::run_game;  // Fix module reference

#[macroquad::main("RustBlocks")]
async fn main() {
    run_game().await;
}