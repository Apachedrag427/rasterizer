use crate::types::{Color, Coordinate2d, Lerp};

pub struct Frame {
	width: usize,
	height: usize,
	data: Vec<Color>,
}

impl Frame {
	pub fn new(width: usize, height: usize) -> Frame {
		Frame {
			width,
			height,
			data: vec![Color::default(); width * height],
		}
	}

	#[inline]
	pub fn get_dimensions(&self) -> Coordinate2d {
		Coordinate2d::new(self.width as isize, self.height as isize)
	}

	#[inline]
	pub fn get_pixel(&self, x: usize, y: usize) -> Option<&Color> {
		self.data.get(y * self.width + x)
	}

	#[inline]
	pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
		if x >= self.width || y >= self.height {
			return;
		}
		self.set_pixel_i(y * self.width + x, color);
	}

	#[inline]
	pub fn set_pixel_i(&mut self, i: usize, color: Color) {
		if i >= self.data.len() {
			return;
		}

		if color.a <= 0. {
			return;
		}
		if color.a >= 1. {
			self.data[i] = color;
			return;
		}

		let mut res = self.data[i].lerp(color, color.a);
		res.a = 1.;
		self.data[i] = res;
	}

	#[inline]
	pub fn get_raw_data(&self) -> &Vec<Color> {
		&self.data
	}
}

mod bulk;
mod line;
mod tri;

mod iter;
pub use iter::{FrameItem, FrameIter};
