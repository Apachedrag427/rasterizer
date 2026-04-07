use crate::frame::{Frame, FrameItem};

pub trait RenderBackend {
	fn begin_rendering(&mut self);
	fn end_rendering(&mut self);
	fn prepare_for_next_frame(&mut self, frame: &Frame);
	fn render_frame(&mut self, frame: Frame);
}

mod stdout;

pub use stdout::StdoutAnsiRenderer;
pub use stdout::StdoutAsciiRenderer;

mod kitty_graphics;

pub use kitty_graphics::KittyEscapeRenderer;
pub use kitty_graphics::KittySHMRenderer;
