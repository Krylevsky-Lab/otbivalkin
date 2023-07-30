use macroquad::{prelude::*, Error};

struct Frog {
	point: (f32, f32),
	size: f32,
}

#[macroquad::main("Arkanoid")]
async fn main() {
	let cookie_texture = load_texture("cookie.png").await.unwrap();
	let frog_texture = load_texture("frog.png").await.unwrap();
	let mut frogs: Vec<Frog> = vec![];

	frogs.push(Frog {
		point: (150.0, 265.0),
		size: 24.0,
	});

	frogs.push(Frog {
		point: (250.0, 465.0),
		size: 36.0,
	});

	loop {
		draw_rectangle(25.0, 100.0, 100.0, 200.0, DARKPURPLE);
		draw_texture(&cookie_texture, 25.0, 100.0, WHITE);

		for frog in frogs.iter() {
			draw_texture(&frog_texture, frog.point.0, frog.point.1, WHITE);
		}

		next_frame().await
	}
}