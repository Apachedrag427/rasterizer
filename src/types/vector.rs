use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
	pub x: f64,
	pub y: f64,
}

impl Vector2 {
	#[inline]
	pub fn new(x: f64, y: f64) -> Vector2 {
		Vector2 { x, y }
	}
	#[inline]
	pub fn zero() -> Vector2 {
		Vector2::new(0., 0.)
	}
	#[inline]
	pub fn one() -> Vector2 {
		Vector2::new(1., 1.)
	}

	#[inline]
	pub fn magnitude(&self) -> f64 {
		(self.x * self.x + self.y * self.y).sqrt()
	}

	#[inline]
	pub fn normalize(&self) -> Vector2 {
		let magnitude = self.magnitude();
		Vector2 {
			x: self.x / magnitude,
			y: self.y / magnitude,
		}
	}

	#[inline]
	pub fn dot(&self, rhs: Vector2) -> f64 {
		(self.x * rhs.x) + (self.y * rhs.y)
	}
}

impl Add<Vector2> for Vector2 {
	type Output = Vector2;
	fn add(self, rhs: Vector2) -> Self::Output {
		Vector2::new(self.x + rhs.x, self.y + rhs.y)
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl Vector3 {
	#[inline]
	pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
		Vector3 { x, y, z }
	}
	#[inline]
	pub fn zero() -> Vector3 {
		Vector3::new(0., 0., 0.)
	}
	#[inline]
	pub fn one() -> Vector3 {
		Vector3::new(1., 1., 1.)
	}
	#[inline]
	pub fn magnitude(&self) -> f64 {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}

	#[inline]
	pub fn normalize(&self) -> Vector3 {
		let magnitude = self.magnitude();
		Vector3 {
			x: self.x / magnitude,
			y: self.y / magnitude,
			z: self.z / magnitude,
		}
	}

	#[inline]
	pub fn dot(&self, rhs: Vector3) -> f64 {
		(self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinate2d {
	pub x: isize,
	pub y: isize,
}

impl Coordinate2d {
	#[inline]
	pub fn new(x: isize, y: isize) -> Coordinate2d {
		Coordinate2d { x, y }
	}
	#[inline]
	pub fn zero() -> Coordinate2d {
		Coordinate2d::new(0, 0)
	}
	#[inline]
	pub fn one() -> Coordinate2d {
		Coordinate2d::new(1, 1)
	}
}

impl Add<Coordinate2d> for Coordinate2d {
	type Output = Coordinate2d;
	fn add(self, rhs: Coordinate2d) -> Self::Output {
		Coordinate2d::new(self.x + rhs.x, self.y + rhs.y)
	}
}

impl Into<Coordinate2d> for Vector2 {
	fn into(self) -> Coordinate2d {
		Coordinate2d {
			x: self.x.round() as isize,
			y: self.y.round() as isize,
		}
	}
}

impl Into<Vector2> for Coordinate2d {
	fn into(self) -> Vector2 {
		Vector2 {
			x: self.x as f64,
			y: self.y as f64,
		}
	}
}
