use super::Frame;
use crate::types::{Color3, Coordinate2d, CoordinateRect, Vector2};

impl Frame {
	// Explanation of Bresenham's Line Algorithm: https://www.youtube.com/watch?v=CceepU1vIKo
	pub fn draw_line(&mut self, start: Vector2, end: Vector2, color: Color3) {
		if (end.x - start.x).abs() > (end.y - start.y).abs() {
			self.draw_line_horizontal(start, end, color);
		} else {
			self.draw_line_vertical(start, end, color);
		}
	}

	pub fn draw_line_int(&mut self, start: Coordinate2d, end: Coordinate2d, color: Color3) {
		if (end.x - start.x).abs() > (end.y - start.y).abs() {
			self.draw_line_horizontal_int(start, end, color);
		} else {
			self.draw_line_vertical_int(start, end, color);
		}
	}

	fn draw_line_horizontal(&mut self, start: Vector2, end: Vector2, color: Color3) {
		self.draw_line_horizontal_int(start.into(), end.into(), color);
	}

	fn draw_line_horizontal_int(
		&mut self,
		mut start: Coordinate2d,
		mut end: Coordinate2d,
		color: Color3,
	) {
		if start.x > end.x {
			(start.x, end.x) = (end.x, start.x);
			(start.y, end.y) = (end.y, start.y);
		}

		if start.y == end.y {
			self.fill_rect_int(
				CoordinateRect {
					position: start,
					dimensions: Coordinate2d::new(end.x - start.x, 1),
				},
				color,
			);
		}

		let dx = end.x - start.x;
		let mut dy = end.y - start.y;

		let dir = if dy < 0 { -1 } else { 1 };
		dy *= dir;

		if dx != 0 {
			let mut y = start.y;
			let mut p = 2 * dy - dx;
			for x in 0..=dx {
				self.set_pixel(start.x as usize + x as usize, y as usize, color);
				if p >= 0 {
					y += dir;
					p -= 2 * dx;
				}
				p += 2 * dy;
			}
		}
	}

	fn draw_line_vertical(&mut self, start: Vector2, end: Vector2, color: Color3) {
		self.draw_line_vertical_int(start.into(), end.into(), color);
	}

	fn draw_line_vertical_int(
		&mut self,
		mut start: Coordinate2d,
		mut end: Coordinate2d,
		color: Color3,
	) {
		if start.y > end.y {
			(start.x, end.x) = (end.x, start.x);
			(start.y, end.y) = (end.y, start.y);
		}

		if start.x == end.x {
			self.fill_rect_int(
				CoordinateRect {
					position: start,
					dimensions: Coordinate2d::new(1, end.y - start.y),
				},
				color,
			);
		}

		let mut dx = end.x - start.x;
		let dy = end.y - start.y;

		let dir = if dx < 0 { -1 } else { 1 };
		dx *= dir;

		if dy != 0 {
			let mut x = start.x;
			let mut p = 2 * dx - dy;
			for y in 0..=dy {
				self.set_pixel(x as usize, start.y as usize + y as usize, color);
				if p >= 0 {
					x += dir;
					p -= 2 * dy;
				}
				p += 2 * dx;
			}
		}
	}
}
