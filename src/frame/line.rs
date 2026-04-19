use super::Frame;
use crate::types::{Color, Coordinate2d, CoordinateRect, Vector2};

impl Frame {
	// Explanation of Bresenham's Line Algorithm: https://www.youtube.com/watch?v=CceepU1vIKo
	pub fn draw_line(&mut self, start: Vector2, end: Vector2, color: Color) {
		self.draw_line_int(start.into(), end.into(), color);
	}

	pub fn draw_line_int(&mut self, start: Coordinate2d, end: Coordinate2d, color: Color) {
		if (end.x - start.x).abs() > (end.y - start.y).abs() {
			self.draw_horizontal_line(start, end, color);
		} else {
			self.draw_vertical_line(start, end, color);
		}
	}

	fn draw_horizontal_line(
		&mut self,
		mut start: Coordinate2d,
		mut end: Coordinate2d,
		color: Color,
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
			return;
		}

		let dx = end.x - start.x;
		let mut dy = end.y - start.y;

		let dir = if dy < 0 { -1 } else { 1 };
		dy *= dir;

		if dx != 0 {
			let mut y = start.y;
			let mut p = 2 * dy - dx;
			for x in 0..=dx {
				if (start.x + x) >= 0 {
					self.set_pixel((start.x + x) as usize, y as usize, color);
				}

				if p >= 0 {
					y += dir;
					p -= 2 * dx;
				}
				p += 2 * dy;
			}
		}
	}

	fn draw_vertical_line(&mut self, mut start: Coordinate2d, mut end: Coordinate2d, color: Color) {
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
			return;
		}

		let mut dx = end.x - start.x;
		let dy = end.y - start.y;

		let dir = if dx < 0 { -1 } else { 1 };
		dx *= dir;

		if dy != 0 {
			let mut x = start.x;
			let mut p = 2 * dx - dy;
			for y in 0..=dy {
				if (start.y + y) >= 0 {
					self.set_pixel(x as usize, (start.y + y) as usize, color);
				}

				if p >= 0 {
					x += dir;
					p -= 2 * dy;
				}
				p += 2 * dx;
			}
		}
	}

	pub fn draw_arrow(
		&mut self,
		start: Vector2,
		end: Vector2,
		chevron_length: f64,
		chevron_angle: f64,
		color: Color,
	) {
		self.draw_arrow_int(
			start.into(),
			end.into(),
			chevron_length,
			chevron_angle,
			color,
		);
	}

	// https://math.stackexchange.com/questions/1314006/drawing-an-arrow
	pub fn draw_arrow_int(
		&mut self,
		start: Coordinate2d,
		end: Coordinate2d,
		chevron_length: f64,
		chevron_angle: f64,
		color: Color,
	) {
		let length = (end - start).magnitude();
		let ratio = chevron_length / length;
		self.draw_line_int(start, end, color);
		let cos_chevron = f64::cos(chevron_angle);
		let sin_chevron = f64::sin(chevron_angle);

		let end_to_start: Vector2 = (start - end).into();
		let xcos = end_to_start.x * cos_chevron;
		let ycos = end_to_start.y * cos_chevron;
		let xsin = end_to_start.x * sin_chevron;
		let ysin = end_to_start.y * sin_chevron;

		let chevron_1 = Coordinate2d::new(
			end.x + (ratio * (xcos + ysin)) as isize,
			end.y + (ratio * (ycos - xsin)) as isize,
		);
		let chevron_2 = Coordinate2d::new(
			end.x + (ratio * (xcos - ysin)) as isize,
			end.y + (ratio * (ycos + xsin)) as isize,
		);

		self.draw_line_int(end, chevron_1, color);
		self.draw_line_int(end, chevron_2, color);
	}
}
