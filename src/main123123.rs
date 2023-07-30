#![feature(slice_flatten)]

mod polygon;

use core::panic;

use macroquad::prelude::*;
use tap::Tap;

struct BodyPoint(u32, u32);
struct DirectionPoint(i32, i32);

const WIDTH: u32 = 40;
const HEIGHT: u32 = 30;

#[derive(Clone, Copy, PartialEq)]
enum PointType {
	Empty,
	Apple,
}

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
	map: [[PointType; WIDTH as usize]; HEIGHT as usize],
	frogs: Vec<Frog>,
	snake: Vec<BodyPoint>,
	snake_direction: DirectionPoint,
}

impl Default for State {
	fn default() -> Self {
		State {
			map: [[PointType::Empty; WIDTH as usize]; HEIGHT as usize],
			frogs: vec![],
			snake: vec![BodyPoint(0, 0)],
			snake_direction: DirectionPoint(1, 0),
		}
	}
}

impl State {
	fn add_apple(&mut self) {
		self.map[rand::gen_range(0, HEIGHT) as usize][rand::gen_range(0, WIDTH) as usize] = PointType::Apple;
		// self.map[0 as usize][2 as usize] = PointType::Apple;
	}

	fn add_frog(&mut self) {
		self.frogs.push(Frog {
			point: BodyPoint(rand::gen_range(0, screen_height() as u32), rand::gen_range(0, screen_width() as u32)),
			size: 5.0,
			state: FrogState::Growing,
		});
	}
}

trait Draw {
	fn draw();
}

#[macroquad::main("Arkanoid")]
async fn main() {
	// const SIZE: f32 = 0.5;
	const SCR_W: f32 = 800.0;
	const SCR_H: f32 = 600.0;
	const BLOCK_W: f32 = SCR_W / WIDTH as f32;
	const BLOCK_H: f32 = SCR_H / HEIGHT as f32;
	request_new_screen_size(800.0, 600.0);
	// set_camera(&Camera2D {
	// 	zoom: vec2(1. / SCR_W * 2., 1. / SCR_H * 2.),
	// 	target: vec2(SCR_W / 2., SCR_H / 2.),
	// 	..Default::default()
	// });
	let mut state = State::default();
	let mut last_time: f64 = 0.0;
	let mut frog_time: f64 = 0.0;

	state.add_apple();

	let frog_texture: Texture2D = load_texture("frog.png").await.unwrap();

	simulate_mouse_with_touch(true);

	loop {
		clear_background(SKYBLUE);

		if is_key_down(KeyCode::Right) {
			state.snake_direction = DirectionPoint(1, 0);
		}
		if is_key_down(KeyCode::Left) {
			state.snake_direction = DirectionPoint(-1, 0);
		}
		if is_key_down(KeyCode::Up) {
			state.snake_direction = DirectionPoint(0, -1);
		}
		if is_key_down(KeyCode::Down) {
			state.snake_direction = DirectionPoint(0, 1);
		}

		// self.frogs = vec![];
		// self.frogs.push(Frog {
		// 	point: BodyPoint(50.0, 100.0),
		// 	size: 5.0,
		// });
		// for frog in state.frogs.iter() {
		// 	if touch.position.x >= (frog.point.0 as f32 - frog.size / 2.0) &&
		// 		touch.position.x <= (frog.point.0 as f32 + frog.size / 2.0) &&
		// 		touch.position.y >= (frog.point.1 as f32 - frog.size / 2.0) &&
		// 		touch.position.y <= (frog.point.1 as f32 + frog.size / 2.0) {
		// 		println!("frog!");
		// 	}
		// }

		if is_mouse_button_pressed(MouseButton::Left) {
			let (x, y) = mouse_position();
			for frog in state.frogs.iter() {
				if x >= (frog.point.0 as f32 - frog.size / 2.0) &&
					x <= (frog.point.0 as f32 + frog.size / 2.0) &&
					y >= (frog.point.1 as f32 - frog.size / 2.0) &&
					y <= (frog.point.1 as f32 + frog.size / 2.0) {
					println!("frog!");
				}
			}
		}

		// for touch in touches() {
		// 	// let (fill_color, size) = match touch.phase {
		// 	// 	TouchPhase::Started => (GREEN, 80.0),
		// 	// 	TouchPhase::Stationary => (WHITE, 60.0),
		// 	// 	TouchPhase::Moved => (YELLOW, 60.0),
		// 	// 	TouchPhase::Ended => (BLUE, 80.0),
		// 	// 	TouchPhase::Cancelled => (BLACK, 80.0),
		// 	// };
		// 	println!("frog!2");
		// 	let TouchPhase::Started = touch.phase else { continue };
		// }

		let delta = get_time();
		let delta_frame = get_frame_time();
		if delta - last_time > 0.09 {
			last_time = delta;
			let last_point = state.snake.last().unwrap();
			let next_point = BodyPoint((last_point.0 as i32 + state.snake_direction.0) as u32, (last_point.1 as i32 + state.snake_direction.1) as u32);
			if let PointType::Apple = state.map[next_point.1 as usize][next_point.0 as usize] {
				state.map[next_point.1 as usize][next_point.0 as usize] = PointType::Empty;
				state.snake.push(next_point);
				state.add_apple();
			} else {
				state.snake.push(next_point);
				state.snake.remove(0);
			}
		}

		if delta - frog_time > 1.5 {
			frog_time = delta;
			state.add_frog();
		}

		state.frogs = state.frogs.into_iter().filter(|it| it.size < 75.0).collect::<Vec<_>>();
		for frog in state.frogs.iter_mut() {
			frog.size += (delta_frame * 25.0) as f32;
		}

		for frog in state.frogs.iter_mut() {
			// draw_circle(frog.point.0 as f32, frog.point.1 as f32, frog.size, DARKGREEN);
			draw_texture_ex(&frog_texture, frog.point.0 as f32 - frog.size / 2.0, frog.point.1 as f32 - frog.size / 2.0, WHITE, DrawTextureParams {
				dest_size: Some(vec2(frog.size, frog.size)),
				..Default::default()
			});
		}

		for (y, row) in state.map.iter().enumerate() {
			for (x, cell) in row.iter().enumerate() {
				if let PointType::Apple = *cell {
					// println!("apple! {} {}", y, x);
					draw_circle(x as f32 * BLOCK_W + BLOCK_W / 2.0, y as f32 * BLOCK_H + BLOCK_H / 2.0, BLOCK_W / 2.0, RED);
				}
			}
		}

		for (index, point) in state.snake.iter().rev().enumerate() {
			if index == 0 {
				match state.snake_direction {
					DirectionPoint(1, 0) => {
						draw_triangle(
							Vec2::new(point.0 as f32 * BLOCK_W, point.1 as f32 * BLOCK_H), 
							Vec2::new(point.0 as f32 * BLOCK_W, point.1 as f32 * BLOCK_H + BLOCK_H), 
							Vec2::new(point.0 as f32 * BLOCK_W + BLOCK_W, point.1 as f32 * BLOCK_H + BLOCK_H / 2.0),
							GREEN,
						);
					},
					DirectionPoint(0, 1) => {
						draw_triangle(
							Vec2::new(point.0 as f32 * BLOCK_W, point.1 as f32 * BLOCK_H), 
							Vec2::new(point.0 as f32 * BLOCK_W + BLOCK_W, point.1 as f32 * BLOCK_H), 
							Vec2::new(point.0 as f32 * BLOCK_W + BLOCK_W / 2.0, point.1 as f32 * BLOCK_H + BLOCK_H),
							GREEN,
						);
					},
					DirectionPoint(-1, 0) => {
						draw_triangle(
							Vec2::new(point.0 as f32 * BLOCK_W + BLOCK_W, point.1 as f32 * BLOCK_H), 
							Vec2::new(point.0 as f32 * BLOCK_W + BLOCK_W, point.1 as f32 * BLOCK_H + BLOCK_H), 
							Vec2::new(point.0 as f32 * BLOCK_W, point.1 as f32 * BLOCK_H + BLOCK_H / 2.0),
							GREEN,
						);
					},
					DirectionPoint(0, -1) => {
						draw_triangle(
							Vec2::new(point.0 as f32 * BLOCK_W, point.1 as f32 * BLOCK_H + BLOCK_H), 
							Vec2::new(point.0 as f32 * BLOCK_W + BLOCK_W, point.1 as f32 * BLOCK_H + BLOCK_H), 
							Vec2::new(point.0 as f32 * BLOCK_W + BLOCK_W / 2.0, point.1 as f32 * BLOCK_H),
							GREEN,
						);
					},
					_ => {},
				}
			} else {
				draw_rectangle(
					point.0 as f32 * BLOCK_W,
					point.1 as f32 * BLOCK_H,
					BLOCK_W, BLOCK_H,
					GREEN,
				);
			}
		}
		
		
		next_frame().await
	}
}