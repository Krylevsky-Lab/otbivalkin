use macroquad::prelude::*;

struct BodyPoint(u32, u32);

enum FrogState {
	Growing,
	Destroying,
	Destroyed,
}

struct Frog {
	point: BodyPoint,
	size: f32,
	state: FrogState,
}

struct State {
	frogs: Vec<Frog>,
}

impl Default for State {
	fn default() -> Self {
		State {
			frogs: vec![],			
		}
	}
}

impl State {
	fn add_frog(&mut self) {
		self.frogs.push(Frog {
			point: BodyPoint(rand::gen_range(0, screen_height() as u32), rand::gen_range(0, screen_width() as u32)),
			size: 5.0,
			state: FrogState::Growing,
		});
	}
}

#[macroquad::main("Arkanoid")]
async fn main() {
	request_new_screen_size(800.0, 600.0);

	let mut state = State::default();

	let frog_texture = load_texture("frog.png").await.unwrap();
	let mut frog_time: f64 = 0.0;

	loop {
		clear_background(SKYBLUE);

		let delta = get_time();
		let delta_frame = get_frame_time();

		if is_mouse_button_pressed(MouseButton::Left) {
			let (x, y) = mouse_position();
			for (index, frog) in state.frogs.iter().enumerate() {
				if x >= (frog.point.0 as f32 - frog.size / 2.0) &&
					x <= (frog.point.0 as f32 + frog.size / 2.0) &&
					y >= (frog.point.1 as f32 - frog.size / 2.0) &&
					y <= (frog.point.1 as f32 + frog.size / 2.0) {
					state.frogs.remove(index);
					break;
				}
			}
		}

		if delta - frog_time > 0.4 {
			frog_time = delta;
			state.add_frog();
		}

		// for (index, frog) in state.frogs.iter().enumerate() {
		// 	if frog.size >= 75.0 {
		// 		state.frogs.remove(index);
		// 		break;
		// 	}
		// }

		state.frogs = state.frogs.into_iter().filter(|it| it.size < 75.0).collect::<Vec<_>>();
		for frog in state.frogs.iter_mut() {
			frog.size += (delta_frame * 25.0) as f32;
		}

		for frog in state.frogs.iter_mut() {
			draw_texture_ex(&frog_texture, frog.point.0 as f32 - frog.size / 2.0, frog.point.1 as f32 - frog.size / 2.0, WHITE, DrawTextureParams {
				dest_size: Some(vec2(frog.size, frog.size)),
				..Default::default()
			});
		}
	
		next_frame().await
	}
}
