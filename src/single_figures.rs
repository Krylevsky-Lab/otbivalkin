use macroquad::prelude::*;

#[macroquad::main("Game")]
async fn main() {
	loop {
		draw_rectangle(100.0, 150.0, 25.0, 50.0, DARKGREEN);
		draw_circle(250.0, 150.0, 25.0, DARKPURPLE);
		next_frame().await
	}
}