use super::Lerp;
use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Color {
	pub r: f64,
	pub g: f64,
	pub b: f64,
	pub a: f64,
}
impl Color {
	#[inline]
	pub fn new(r: f64, g: f64, b: f64) -> Color {
		Color { r, g, b, a: 1. }
	}

	#[inline]
	pub fn rgba(r: f64, g: f64, b: f64, a: f64) -> Color {
		Color { r, g, b, a }
	}

	#[inline]
	pub fn white() -> Color {
		Color::new(1., 1., 1.)
	}
	#[inline]
	pub fn black() -> Color {
		Color::new(0., 0., 0.)
	}
	#[inline]
	pub fn red() -> Color {
		Color::new(1., 0., 0.)
	}
	#[inline]
	pub fn green() -> Color {
		Color::new(0., 1., 0.)
	}
	#[inline]
	pub fn blue() -> Color {
		Color::new(0., 0., 1.)
	}

	#[inline]
	pub fn transparent() -> Color {
		Color::rgba(0., 0., 0., 0.)
	}

	#[inline]
	pub fn invert(&self) -> Color {
		Color {
			r: 1. - self.r,
			g: 1. - self.g,
			b: 1. - self.b,
			a: self.a,
		}
	}

	#[inline]
	pub fn from_value(n: f64) -> Color {
		Color {
			r: n,
			g: n,
			b: n,
			a: 1.,
		}
	}

	#[inline]
	pub fn get_rgb(&self) -> (u8, u8, u8) {
		(
			(self.r * 255.) as u8,
			(self.g * 255.) as u8,
			(self.b * 255.) as u8,
		)
	}

	#[inline]
	pub fn get_compact_rgb(&self) -> u32 {
		(self.b * 255.) as u32 | (((self.g * 255.) as u32) << 8) | (((self.r * 255.) as u32) << 16)
	}

	#[inline]
	pub fn get_lightness(&self) -> f64 {
		(0.21 * self.r) + (0.72 * self.g) + (0.07 * self.b).clamp(0., 1.)
	}
}

impl Lerp<Color> for Color {
	type Output = Color;
	fn lerp(&self, rhs: Color, a: f64) -> Self::Output {
		Self {
			r: self.r.lerp(rhs.r, a),
			g: self.g.lerp(rhs.g, a),
			b: self.b.lerp(rhs.b, a),
			a: self.a.lerp(rhs.a, a),
		}
	}
}

impl Add<Color> for Color {
	type Output = Color;

	fn add(self, rhs: Color) -> Self::Output {
		Color::rgba(
			self.r + rhs.r,
			self.g + rhs.g,
			self.b + rhs.b,
			self.a + rhs.a,
		)
	}
}

impl Mul<f64> for Color {
	type Output = Color;

	fn mul(self, rhs: f64) -> Self::Output {
		Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
	}
}

impl Div<f64> for Color {
	type Output = Color;

	fn div(self, rhs: f64) -> Self::Output {
		Color::new(self.r / rhs, self.g / rhs, self.b / rhs)
	}
}

impl Default for Color {
	fn default() -> Self {
		Color {
			r: 0.,
			g: 0.,
			b: 0.,
			a: 0.,
		}
	}
}
