use std::f32::consts::PI;
mod calculate_circle_point;
use macroquad::prelude::*;
use crate::calculate_circle_point::calculate_circle_point;

struct Cursor {
	size: (f32, f32),
	from: (f32, f32, f32),
	to: (f32, f32, f32),
	progress: f32, // from 0 to 1
}

trait Draw {
	fn draw(&self, texture: &Texture2D);
}

fn lerp(start: (f32, f32), stop: (f32, f32), progress: f32) -> Vec2 {
	return progress * (vec2(stop.0, stop.1) - vec2(start.0, start.1)) + vec2(start.0, start.1);
}

impl Cursor {
	fn current_point(&self) -> Vec2 {
		lerp((self.from.0, self.from.1), (self.to.0, self.to.1), self.progress)
	}
}

impl Draw for Cursor {
    fn draw(&self, texture: &Texture2D) {
		let point = self.current_point();
		draw_texture_ex(&texture, point.x, point.y, WHITE, DrawTextureParams {
			dest_size: Some(vec2(self.size.0, self.size.1)),
			pivot: Some(vec2(point.x, point.y)),
			rotation: self.from.2 + 3.0 * PI / 4.0 + PI,
			..Default::default()
			// source: (),
			// flip_x: (),
			// flip_y: (),
		});
	
		// draw_circle(self.from.0, self.from.1, 2.0, RED);
		// draw_circle(self.to.0, self.to.1, 2.0, PURPLE);
    }
}

#[macroquad::main("Game")]
async fn main() {
	request_new_screen_size(800.0, 600.0);

	let cookie_texture = load_texture("cookie.png").await.unwrap();
	let cursor_texture = load_texture("cursor.png").await.unwrap();

	// 741,415866247
	// 24,7138622082

	let cookie_perimetr = 741.415866247;
	let max_cursors = 30;
	let size = cookie_perimetr / max_cursors as f32;

	let mut cursors = vec![];
	let x_center = 400.0;
	let y_center = 300.0;
	let cookie_half_width = cookie_texture.width() / 2.0;

	for i in 0..max_cursors {
		cursors.push(Cursor {
			size: (size, size),
			from: calculate_circle_point(i, max_cursors, cookie_half_width + 10.0, x_center, y_center),
			to: calculate_circle_point(i, max_cursors, cookie_half_width, x_center, y_center),
			progress: 0.0,
		});
	}
	
	let mut current_cursor_anim_index = 0;

	loop {
		clear_background(SKYBLUE);

		draw_texture(&cookie_texture, 400.0 - cookie_texture.width() / 2.0, 300.0 - cookie_texture.height() / 2.0, WHITE);

		let current_cursor = cursors.get_mut(current_cursor_anim_index).unwrap();

		current_cursor.progress += get_frame_time() * 2.0;
		if current_cursor.progress > 1.0 {
			current_cursor.progress = 0.0;
			if current_cursor_anim_index == cursors.len() - 1 {
				current_cursor_anim_index = 0;
			} else {
				current_cursor_anim_index += 1;
			}
		}

		for cursor in cursors.iter_mut() {
			// cursor.progress += get_frame_time() * 2.0;
			// if cursor.progress > 1.0 {
			// 	cursor.progress = 0.0;
			// }
			cursor.draw(&cursor_texture);
		}
		// for i in 0..max_cursors {
		// 	draw_circle_point(&cursor_texture, &cookie_texture, i, max_cursors, 118.0, 400.0, 300.0);
		// }

		next_frame().await
	}
}