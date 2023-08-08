use std::{rc::Rc, cell::Cell};

use macroquad::prelude::{*, animation::*};

fn explode(x: f32, y: f32, explode_x: &mut f32, explode_y: &mut f32, explode_sprite: &mut AnimatedSprite) {
	*explode_x = x;
	*explode_y = y;
	explode_sprite.set_animation(0);
	explode_sprite.playing = true;
}

#[macroquad::main("Game")]
async fn main() {
	request_new_screen_size(800.0, 600.0);
	let mut explode_x = 0.0;
	let mut explode_y = 0.0;
	let explode_texture = load_texture("sprites/explode.png").await.unwrap();

	let explode_width = 256;
	let explode_height = 256;

	let mut explode_sprite = AnimatedSprite::new(
		explode_width,
		explode_height,
		&[
			Animation {
				name: "general".to_string(),
				row: 0,
				frames: 10,
				fps: 24,
			},
		],
		false,
		false,
	);

	

	loop {
		if is_mouse_button_pressed(MouseButton::Left) {
			let (x, y) = mouse_position();
			// explode(x, y, &mut explode_x, &mut explode_y, &mut explode_sprite);
			explode_x = x;
			explode_y = y;
			explode_sprite.set_animation(0);
			explode_sprite.playing = true;
		}

		if explode_sprite.playing {		
			draw_texture_ex(
				&explode_texture,
				explode_x - explode_width as f32 / 2.0,
				explode_y - explode_height as f32 / 2.0,
				WHITE,
				DrawTextureParams {
					source: Some(explode_sprite.frame().source_rect),
					dest_size: Some(explode_sprite.frame().dest_size),
					..Default::default()
				}
			);
		}

		explode_sprite.update();
		next_frame().await
	}
}