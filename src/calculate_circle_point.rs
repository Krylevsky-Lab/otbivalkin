// draw_circle(new_x, new_y, 2.0, RED);

// draw_texture_ex(&cursor_texture, new_x, new_y, WHITE, DrawTextureParams {
// 	dest_size: Some(vec2(size, size)),
// 	pivot: Some(vec2(new_x, new_y)),
// 	rotation: angle + 3.0 * PI / 4.0 + PI,
// 	..Default::default()
// 	// source: (),
// 	// flip_x: (),
// 	// flip_y: (),
// });

use std::f32::consts::PI;

pub fn calculate_circle_point(index: i32, points: i32, abra: f32, radius: f32, x: f32, y: f32) -> (f32, f32, f32)
{
	let slice = 2.0 * PI / points as f32;
	let angle = slice * index as f32;
	let new_x = x + radius * angle.cos();
	let new_y = y + radius * angle.sin();

	(new_x, new_y, angle)
}
