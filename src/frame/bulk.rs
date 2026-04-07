use super::Frame;
use crate::types::{Color3, Coordinate2d, CoordinateRect, Rect};

impl Frame {
	pub fn fill_rect(&mut self, rect: Rect, color: Color3) {
		self.fill_rect_int(rect.into(), color);
	}

	pub fn fill_rect_int(&mut self, rect: CoordinateRect, color: Color3) {
		let start: Coordinate2d = rect.position.into();
		let mut end: Coordinate2d = (rect.position + rect.dimensions).into();

		// A scale of 1 means 0 offset.
		end.x -= 1;
		end.y -= 1;

		if start.x == end.x {
			let mut i = start.y * (self.width as isize) + start.x;
			for _y in start.y..=end.y {
				self.data[i as usize] = color;

				// Go down a row
				i += self.width as isize;
			}
		}

		let mut i;
		for y in start.y..=end.y {
			i = y * (self.width as isize) + start.x;
			for _x in start.x..=end.x {
				self.data[i as usize] = color;
				i += 1;
			}
		}
	}

	pub fn clear(&mut self, color: Color3) {
		self.data.fill(color);
	}

	pub fn callback_fill<T: Fn(usize, usize) -> Color3>(&mut self, callback: T) {
		// Use a separate index here to avoid having to recompute it for every pixel
		// The caveat is that I **MUST** loop row by row for the index to line up
		let mut i = 0;
		for y in 0..self.height {
			for x in 0..self.width {
				self.data[i] = callback(x, y);
				i += 1;
			}
		}
	}

	pub fn callback_update<T: Fn(usize, usize, Color3) -> Color3>(&mut self, callback: T) {
		// Same index rules as callback_fill
		let mut i = 0;
		for y in 0..self.height {
			for x in 0..self.width {
				self.data[i] = callback(x, y, self.data[i]);
				i += 1;
			}
		}
	}
}
