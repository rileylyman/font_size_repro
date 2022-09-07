use macroquad::prelude::*;

#[macroquad::main("Foo")]
async fn main() {
    request_new_screen_size(400.0, 400.0);

    loop {
        draw_rectangle(0.0, 0.0, 200.0, 200.0, WHITE);
        draw_text("Testing 123", 10.0, 100.0, 32.0, BLACK);

        next_frame().await;
    }
}
