use std::ops::{Div, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Color3 {
	pub r: f64,
	pub g: f64,
	pub b: f64,
}
impl Color3 {
	#[inline]
	pub fn new(r: f64, g: f64, b: f64) -> Color3 {
		Color3 { r, g, b }
	}

	#[inline]
	pub fn white() -> Color3 {
		Color3::new(1., 1., 1.)
	}
	#[inline]
	pub fn black() -> Color3 {
		Color3::new(0., 0., 0.)
	}
	#[inline]
	pub fn red() -> Color3 {
		Color3::new(1., 0., 0.)
	}
	#[inline]
	pub fn green() -> Color3 {
		Color3::new(0., 1., 0.)
	}
	#[inline]
	pub fn blue() -> Color3 {
		Color3::new(0., 0., 1.)
	}

	#[inline]
	pub fn invert(&self) -> Color3 {
		Color3 {
			r: 1. - self.r,
			g: 1. - self.g,
			b: 1. - self.b,
		}
	}

	#[inline]
	pub fn from_value(n: f64) -> Color3 {
		Color3 { r: n, g: n, b: n }
	}

	#[inline]
	pub fn get_lightness(&self) -> f64 {
		(0.21 * self.r) + (0.72 * self.g) + (0.07 * self.b).clamp(0., 1.)
	}
}

impl Mul<f64> for Color3 {
	type Output = Color3;

	fn mul(self, rhs: f64) -> Self::Output {
		Color3::new(self.r * rhs, self.g * rhs, self.b * rhs)
	}
}

impl Div<f64> for Color3 {
	type Output = Color3;

	fn div(self, rhs: f64) -> Self::Output {
		Color3::new(self.r / rhs, self.g / rhs, self.b / rhs)
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
