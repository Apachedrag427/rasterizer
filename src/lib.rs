#[derive(Debug)]
pub struct Vector2 {
	pub x: f64,
	pub y: f64,
}

impl Vector2 {
	pub fn new(x: f64, y: f64) -> Vector2 {
		Vector2 { x, y }
	}

	pub fn magnitude(&self) -> f64 {
		(self.x * self.x + self.y * self.y).sqrt()
	}

	pub fn normalize(&self) -> Vector2 {
		let magnitude = self.magnitude();
		Vector2 {
			x: self.x / magnitude,
			y: self.y / magnitude,
		}
	}

	pub fn dot(&self, rhs: Vector2) -> f64 {
		(self.x * rhs.x) + (self.y * rhs.y)
	}
}

#[derive(Debug)]
pub struct Vector3 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl Vector3 {
	pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
		Vector3 { x, y, z }
	}
	pub fn magnitude(&self) -> f64 {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}

	pub fn normalize(&self) -> Vector3 {
		let magnitude = self.magnitude();
		Vector3 {
			x: self.x / magnitude,
			y: self.y / magnitude,
			z: self.z / magnitude,
		}
	}

	pub fn dot(&self, rhs: Vector3) -> f64 {
		(self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
	}
}

#[derive(Debug)]
pub struct Triangle2d {
	pub point1: Vector2,
	pub point2: Vector2,
	pub point3: Vector2,
}

#[derive(Debug)]
pub struct Triangle3d {
	pub point1: Vector3,
	pub point2: Vector3,
	pub point3: Vector3,
}

fn clampf(v: f64, min: f64, max: f64) -> f64 {
	if min > v {
		min
	} else if max < v {
		max
	} else {
		v
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Color3 {
	pub r: f64,
	pub g: f64,
	pub b: f64,
}
impl Color3 {
	pub fn new(r: f64, g: f64, b: f64) -> Color3 {
		Color3 { r, g, b }
	}

	pub fn white() -> Color3 {
		Color3::new(1., 1., 1.)
	}
	pub fn black() -> Color3 {
		Color3::new(0., 0., 0.)
	}
	pub fn red() -> Color3 {
		Color3::new(1., 0., 0.)
	}
	pub fn green() -> Color3 {
		Color3::new(0., 1., 0.)
	}
	pub fn blue() -> Color3 {
		Color3::new(0., 0., 1.)
	}

	pub fn get_lightness(&self) -> f64 {
		clampf((0.21 * self.r) + (0.72 * self.g) + (0.07 * self.b), 0., 1.)
	}
}

impl Default for Color3 {
	fn default() -> Self {
		Color3 {
			r: 0.,
			g: 0.,
			b: 0.,
		}
	}
}

pub enum RasterError {
	IndexOutOfBoundsError,
}

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

	pub fn get_pixel(&self, x: usize, y: usize) -> Option<&Color3> {
		self.data.get(y * self.width + x)
	}

	pub fn set_pixel(&mut self, x: usize, y: usize, color: Color3) -> Result<(), RasterError> {
		if x >= self.width || y >= self.height {
			return Err(RasterError::IndexOutOfBoundsError);
		}
		self.data[y * self.width + x] = color;

		Ok(())
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
