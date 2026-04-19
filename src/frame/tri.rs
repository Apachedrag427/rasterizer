use super::*;
use crate::types::{Coordinate2d, CoordinateTriangle2d, Triangle2d, Winding};

fn edge_cross(start: Coordinate2d, end: Coordinate2d, point: Coordinate2d) -> isize {
	(end - start).cross2d(point - start)
}

impl Frame {
	pub fn draw_wireframe_tri(&mut self, tri: Triangle2d, color: Color) {
		self.draw_wireframe_tri_int(tri.into(), color);
	}

	pub fn draw_wireframe_tri_int(&mut self, tri: CoordinateTriangle2d, color: Color) {
		self.draw_line_int(tri.0, tri.1, color);
		self.draw_line_int(tri.1, tri.2, color);
		self.draw_line_int(tri.2, tri.0, color);
	}

	pub fn draw_tri(&mut self, tri: Triangle2d, color: Color) {
		self.draw_tri_int(tri.into(), color);
	}

	pub fn draw_tri_int(&mut self, tri: CoordinateTriangle2d, color: Color) {
		let mut bounds = tri.get_bounds_rect();
		if bounds.position.x < 0 {
			bounds.dimensions.x += bounds.position.x;
			bounds.position.x = 0;
		}
		if bounds.position.x + bounds.dimensions.x >= self.width as isize {
			bounds.dimensions.x += self.width as isize - (bounds.position.x + bounds.dimensions.x);
		}
		let mut frame = Frame::new(
			bounds.dimensions.x.try_into().unwrap(),
			bounds.dimensions.y.try_into().unwrap(),
		);
		let winding = tri.get_winding();
		frame.callback_fill(move |x, y| {
			let point = Coordinate2d::new(
				bounds.position.x + x as isize,
				bounds.position.y + y as isize,
			);
			let w0 = edge_cross(tri.0, tri.1, point);
			let w1 = edge_cross(tri.1, tri.2, point);
			let w2 = edge_cross(tri.2, tri.0, point);

			if (winding == Winding::CW && w0 >= 0 && w1 >= 0 && w2 >= 0)
				|| (winding == Winding::CCW && w0 <= 0 && w1 <= 0 && w2 <= 0)
			{
				color
			} else {
				Color::transparent()
			}
		});
		self.draw_frame_int(bounds.position, frame);
	}
}
