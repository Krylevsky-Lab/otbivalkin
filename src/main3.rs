#![feature(slice_flatten)]

mod polygon;

use core::panic;

use macroquad::prelude::*;

#[derive(PartialEq, Debug)]
enum Side {
	Top,
	Bottom,
	Left,
	Right,
}

#[derive(PartialEq, Debug)]
enum Position {
	Side(Side),
	Corner(Side, Side),
}

#[derive(PartialEq)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

struct GameState {
	size: f32,
	top: f32,
	bottom: f32,
	left: f32,
	right: f32,
	line_width: f32,
	position: Position,
	x: f32,
	y: f32,
	speed: f32,
	direction: Option<Direction>,
	lines: Vec<Rectangle>,
}

struct Point {
	x: f32,
	y: f32,
}

struct Rectangle {
	from: Point,
	to: Point
}

const SIZE: f32 = 0.5;
const SCR_W: f32 = 24.0;
const SCR_H: f32 = 20.0;

impl Default for GameState {
    fn default() -> Self {
        Self {
			size: SIZE,
			top: SIZE,
			bottom: SCR_H - SIZE,
			left: SIZE,
			right: SCR_W - SIZE,
			line_width: 0.4,
			position: Position::Side(Side::Top),
			speed: 12.0,
			x: SIZE * 2.0,
			y: SIZE,
			direction: None,
			lines: vec![],
		}
    }
}

impl GameState {
	fn position(&self) -> Position {
		if self.x == self.left {
			if self.y == self.top {
				Position::Corner(Side::Left, Side::Top)
			} else if self.y == self.bottom {
				Position::Corner(Side::Left, Side::Bottom)
			} else {
				Position::Side(Side::Left)
			}
		} else if self.x == self.right { 
			if self.y == self.top {
				Position::Corner(Side::Right, Side::Top)
			} else if self.y == self.bottom {
				Position::Corner(Side::Right, Side::Bottom)
			} else {
				Position::Side(Side::Right)
			}
		} else if self.y == self.top { 
			Position::Side(Side::Top)
		} else if self.y == self.bottom {
			Position::Side(Side::Bottom)
		} else {
			panic!();
		}
	}

	fn move_fishka_x(&mut self, value: f32) {
		if self.x + value >= self.right {
			self.x = self.right;
			self.auto_move_check_end();
		} else if self.x + value <= self.left {
			self.x = self.left;
			self.auto_move_check_end();
		} else {
			self.x += value;
			if let Some(line) = self.lines.last_mut() {
				line.to = Point { x: self.x, y: self.y };
			}
		}
	}

	fn move_fishka_y(&mut self, value: f32) {
		if self.y + value >= self.bottom {
			self.y = self.bottom;
			self.auto_move_check_end();
		} else if self.y + value <= self.top {
			self.y = self.top;
			self.auto_move_check_end();
		} else {
			self.y += value;
			if let Some(line) = self.lines.last_mut() {
				line.to = Point { x: self.x, y: self.y };
			}
		}
	}

	fn move_fishka_auto(&mut self, direction: Direction) {
		self.direction = Some(direction);
		self.lines.push(Rectangle { from: Point { x: self.x, y: self.y }, to: Point { x: self.x, y: self.y } });
	}

	fn auto_move_check_end(&mut self) {
		if self.direction.is_some() {
			self.direction = None;
			// Отрисовать блоки картинки!
			self.lines.clear();
		}
	}
}

#[macroquad::main("Arkanoid")]
async fn main() {
	const BLOCKS_W: usize = 10;
	const BLOCKS_H: usize = 10;


	let mut state = GameState::default();

	let mut blocks: [[bool; BLOCKS_W]; BLOCKS_H] = [[true; BLOCKS_W]; BLOCKS_H];
	let mut ball_x = 12.;
	let mut ball_y = 7.;
	let mut dx = 3.5;
	let mut dy = -3.5;
	let mut platform_x = 10.;
	let mut stick = true;
	let platform_width = 5.;
	let platform_height = 0.2;

	// build camera with following coordinate system:
	// (0., 0)     .... (SCR_W, 0.)
	// (0., SCR_H) .... (SCR_W, SCR_H)
	set_camera(&Camera2D {
		zoom: vec2(1. / SCR_W * 2., 1. / SCR_H * 2.),
		target: vec2(SCR_W / 2., SCR_H / 2.),
		..Default::default()
	});

	loop {
		clear_background(SKYBLUE);

		let delta = get_frame_time();

		if state.lines.is_empty() {
			println!("{:#?}", state.position());
			match state.position() {
				Position::Side(side) => {
					match side {
						Side::Top => {
							if is_key_down(KeyCode::Right) {
								state.move_fishka_x(state.speed * delta)
							}
							if is_key_down(KeyCode::Left) {
								state.move_fishka_x(-state.speed * delta);
							}
							if is_key_down(KeyCode::Down) {
								state.move_fishka_auto(Direction::Down);
							}
						}
						Side::Bottom => {
							if is_key_down(KeyCode::Right) {
								state.move_fishka_x(state.speed * delta);
							}
							if is_key_down(KeyCode::Left) {
								state.move_fishka_x(-state.speed * delta);
							}
							if is_key_down(KeyCode::Up) {
								state.move_fishka_auto(Direction::Up);
							}
						},
						Side::Left => {
							if is_key_down(KeyCode::Right) {
								state.move_fishka_auto(Direction::Right);
							}
							if is_key_down(KeyCode::Up) {
								state.move_fishka_y(-state.speed * delta);
							}
							if is_key_down(KeyCode::Down) {
								state.move_fishka_y(state.speed * delta);
							}
						},
						Side::Right => {
							if is_key_down(KeyCode::Left) {
								state.move_fishka_auto(Direction::Left);
							}
							if is_key_down(KeyCode::Up) {
								state.move_fishka_y(-state.speed * delta);
							}
							if is_key_down(KeyCode::Down) {
								state.move_fishka_y(state.speed * delta);
							}
						}
					}
					
				}, 
				Position::Corner(one, two) => {
					match one {
						Side::Left => {
							match two {
								Side::Top => {
									if is_key_down(KeyCode::Right) {
										state.move_fishka_x(state.speed * delta);
									}
									if is_key_down(KeyCode::Down) {
										state.move_fishka_y(state.speed * delta);
									}
								},
								Side::Bottom => {
									if is_key_down(KeyCode::Right) {
										state.move_fishka_x(state.speed * delta);
									}
									if is_key_down(KeyCode::Up) {
										state.move_fishka_y(-state.speed * delta);
									}
								},
								_ => {},
							}
						},
						Side::Right => {
							match two {
								Side::Top => {
									if is_key_down(KeyCode::Left) {
										state.move_fishka_x(-state.speed * delta);
									}
									if is_key_down(KeyCode::Down) {
										state.move_fishka_y(state.speed * delta);
									}
								},
								Side::Bottom => {
									if is_key_down(KeyCode::Left) {
										state.move_fishka_x(-state.speed * delta);
									}
									if is_key_down(KeyCode::Up) {
										state.move_fishka_y(-state.speed * delta);
									}
								},
								_ => {},
							}
						},
						_ => {},
					}
				}
			}
		} else if let Some(direction) = state.direction.as_ref() {
			match direction {
				Direction::Up => state.move_fishka_y(-state.speed * delta),
				Direction::Down => state.move_fishka_y(state.speed * delta),
				Direction::Left => state.move_fishka_x(-state.speed * delta),
				Direction::Right => state.move_fishka_x(state.speed * delta),
			}
		}

		for line in state.lines.iter() {
			draw_line(
				line.from.x,
				line.from.y,
				line.to.x,
				line.to.y,
				state.line_width,
				DARKPURPLE,
			);
		}

		draw_circle(state.x, state.y, state.size, RED);
		// if stick == false {
		// 	ball_x += dx * delta;
		// 	ball_y += dy * delta;
		// } else {
		// 	let (font_size, font_scale, font_aspect) = camera_font_scale(1.);
		// 	let text_params = TextParams {
		// 		font_size,
		// 		font_scale,
		// 		font_scale_aspect: font_aspect,
		// 		..Default::default()
		// 	};
		// 	draw_text_ex(
		// 		"Press space to start",
		// 		SCR_W / 2. - 5.,
		// 		SCR_H / 2.,
		// 		text_params,
		// 	);

		// 	ball_x = platform_x;
		// 	ball_y = SCR_H - 0.5;

		// 	stick = !is_key_down(KeyCode::Space);
		// }

		// if ball_x <= 0. || ball_x > SCR_W {
		// 	dx *= -1.;
		// }
		// if ball_y <= 0.
		// 	|| (ball_y > SCR_H - platform_height - 0.15 / 2.
		// 		&& ball_x >= platform_x - platform_width / 2.
		// 		&& ball_x <= platform_x + platform_width / 2.)
		// {
		// 	dy *= -1.;
		// }
		// if ball_y >= SCR_H {
		// 	ball_y = 10.;
		// 	dy = -dy.abs();
		// 	stick = true;
		// }

		// for j in 0..BLOCKS_H {
		// 	for i in 0..BLOCKS_W {
		// 		if blocks[j][i] {
		// 			let block_w = SCR_W / BLOCKS_W as f32;
		// 			let block_h = 7.0 / BLOCKS_H as f32;
		// 			let block_x = i as f32 * block_w + 0.05;
		// 			let block_y = j as f32 * block_h + 0.05;

		// 			draw_rectangle(block_x, block_y, block_w - 0.1, block_h - 0.1, DARKBLUE);
		// 			if ball_x >= block_x
		// 				&& ball_x < block_x + block_w
		// 				&& ball_y >= block_y
		// 				&& ball_y < block_y + block_h
		// 			{
		// 				dy *= -1.;
		// 				blocks[j][i] = false;
		// 			}
		// 		}
		// 	}
		// }

		// draw_circle(ball_x, ball_y, 0.2, RED);
		// draw_rectangle(
		// 	platform_x - platform_width / 2.,
		// 	SCR_H - platform_height,
		// 	platform_width,
		// 	platform_height,
		// 	DARKPURPLE,
		// );

		next_frame().await
	}
}