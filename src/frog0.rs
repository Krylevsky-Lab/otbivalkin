use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
	let frog_texture: Texture2D = load_texture("frog.png").await.unwrap();
	loop {
		draw_texture(&frog_texture, 50, 100);
		draw_texture(&frog_texture, 100, 100);
		draw_texture(&frog_texture, 150, 100);
		next_frame().await
	}
}