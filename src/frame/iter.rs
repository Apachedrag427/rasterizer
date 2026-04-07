use super::Frame;
use crate::types::Color3;

pub enum FrameItem {
	Pixel(usize, usize, Color3),
	LineEnd,
}

pub struct FrameIter {
	data: Vec<Color3>,

	// Height is unnecessary because `Vec.get` will catch it if it goes down too many rows.
	width: usize,

	current_x: usize,
	current_y: usize,
	current_i: usize,

	// Functions as a flag that, when set, will override the next iteration to return FrameItem::LineEnd
	on_line_end: bool,
}

impl Iterator for FrameIter {
	type Item = FrameItem;

	fn next(&mut self) -> Option<Self::Item> {
		if self.on_line_end {
			self.on_line_end = false;
			return Some(FrameItem::LineEnd);
		}

		let current = match self.data.get(self.current_i) {
			Some(color) => color,
			None => return None,
		};

		let current_x = self.current_x;
		let current_y = self.current_y;

		self.current_i += 1;
		self.current_x += 1;
		if self.current_x >= self.width {
			self.current_x = 0;
			self.current_y += 1;

			self.on_line_end = true;
		}

		Some(FrameItem::Pixel(current_x, current_y, current.clone()))
	}
}

impl IntoIterator for Frame {
	type Item = FrameItem;
	type IntoIter = FrameIter;

	fn into_iter(self) -> Self::IntoIter {
		FrameIter {
			data: self.data,
			width: self.width,
			current_x: 0,
			current_y: 0,
			current_i: 0,
			on_line_end: false,
		}
	}
}
