use crate::types::Color3;

pub struct Frame {
	width: usize,
	height: usize,
	data: Vec<Color3>,
}

impl Frame {
	pub fn new(width: usize, height: usize) -> Frame {
		Frame {
			width,
			height,
			data: vec![Color3::default(); width * height],
		}
	}

	pub fn get_dimensions(&self) -> (usize, usize) {
		(self.width, self.height)
	}

	pub fn get_pixel(&self, x: usize, y: usize) -> Option<&Color3> {
		self.data.get(y * self.width + x)
	}

	pub fn set_pixel(&mut self, x: usize, y: usize, color: Color3) {
		self.data[y * self.width + x] = color;
	}
}

mod bulk;
mod line;

mod iter;
pub use iter::{FrameItem, FrameIter};
