use macroquad::prelude::{Circle, PURPLE, Vec2};
use macroquad::shapes::draw_triangle;
use triangular::Triangulation;
use triangulate::{formats};
use triangulate::{PolygonList, ListFormat};

struct Polygon {
	lines: Vec<(f32, f32)>,
	triangles: Vec<[usize;3]>,
}

impl Polygon {
	fn compute_triangles(&mut self) {
		let triangles = Triangulation::new(&self.lines).build().chunks(3).map(|it| {
			let v = [it[0], it[1], it[2]];
			v
		}).collect::<Vec<[usize;3]>>();
		self.triangles = triangles;
	}

	fn draw(&self) {
		for triangle_indice in self.triangles.iter() {
			let t1 = self.lines[triangle_indice[0]];
			let t2 = self.lines[triangle_indice[0]];
			let t3 = self.lines[triangle_indice[0]];
			
			draw_triangle(Vec2::new(t1.0, t1.1), Vec2::new(t2.0, t2.1), Vec2::new(t3.0, t3.1), PURPLE);
		}
	}
}