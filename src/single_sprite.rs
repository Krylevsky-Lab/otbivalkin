use macroquad::prelude::*;

#[macroquad::main("Game")]
async fn main() {
	let frog_texture = load_texture("frog.png").await.unwrap();
	let cursor_texture = load_texture("cursor.png").await.unwrap();

	loop {
		clear_background(SKYBLUE);
		draw_texture(&frog_texture, 150.0, 25.0, WHITE);
		next_frame().await
	}
}