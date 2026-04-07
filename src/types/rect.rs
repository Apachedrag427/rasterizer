use crate::types::vector::*;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
	pub position: Vector2,
	pub dimensions: Vector2,
}

#[derive(Debug, Clone, Copy)]
pub struct CoordinateRect {
	pub position: Coordinate2d,
	pub dimensions: Coordinate2d,
}

impl Into<CoordinateRect> for Rect {
	fn into(self) -> CoordinateRect {
		CoordinateRect {
			position: self.position.into(),
			dimensions: self.dimensions.into(),
		}
	}
}

impl Into<Rect> for CoordinateRect {
	fn into(self) -> Rect {
		Rect {
			position: self.position.into(),
			dimensions: self.dimensions.into(),
		}
	}
}
