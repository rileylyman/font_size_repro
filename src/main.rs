use macroquad::prelude::*;

const FONT_WIDTH: f32 = 118.0;
static mut FONT_SIZE: f32 = 32.0;

#[macroquad::main("Foo")]
async fn main() {
    request_new_screen_size(400.0, 400.0);

    let font = load_ttf_font("Lato-Regular.ttf").await.unwrap();
    let dims = measure_text("A b C", Some(font), unsafe { FONT_SIZE as u16 }, 1.0);
    info!("dims.width={}", dims.width);
    let scale = FONT_WIDTH / dims.width;
    unsafe {
        FONT_SIZE *= scale;
    }

    loop {
        draw_rectangle(0.0, 0.0, 200.0, 200.0, WHITE);
        let params = TextParams {
            font,
            font_size: unsafe { FONT_SIZE as u16 },
            color: BLACK,
            ..Default::default()
        };
        draw_text_ex("Testing 123", 10.0, 100.0, params);
        draw_circle(100.0, 100.0, 30.0, RED);

        next_frame().await;
    }
}
