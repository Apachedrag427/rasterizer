use rasterizer::{Frame, FrameItem};

pub trait Renderer {
	fn begin_rendering(&mut self);
	fn end_rendering(&mut self);
	fn prepare_for_next_frame(&mut self, frame: &Frame);
	fn render_frame(&mut self, frame: Frame);
}

pub mod stdout_grayscale_ascii_renderer {
	use super::*;
	static LIGHTNESS_CHARACTERS: [char; 10] = [' ', '.', ':', '-', '+', '=', '*', '@', '%', '#'];

	pub struct StdoutGrayscaleAsciiRenderer;

	impl Renderer for StdoutGrayscaleAsciiRenderer {
		fn begin_rendering(&mut self) {
			// Hide cursor
			print!("\x1b[?25l")
		}

		fn end_rendering(&mut self) {
			// Show cursor
			print!("\x1b[?25h")
		}

		fn prepare_for_next_frame(&mut self, _frame: &Frame) {
			// Return cursor to "home position"
			print!("\x1b[1;1H")
		}

		fn render_frame(&mut self, frame: Frame) {
			let mut result = String::new();
			for item in frame {
				match item {
					FrameItem::Pixel(_x, _y, color) => {
						let lightness = color.get_lightness();
						result.push(
							LIGHTNESS_CHARACTERS[(lightness
								* ((LIGHTNESS_CHARACTERS.len() - 1) as f64)
								+ 0.5)
								.floor() as usize],
						);
					}
					FrameItem::LineEnd => result.push('\n'),
				}
			}
			println!("{}", result);
		}
	}
}

pub mod stdout_color_ansi_renderer {
	use super::*;

	pub struct StdoutColorAnsiRenderer;

	impl Renderer for StdoutColorAnsiRenderer {
		fn begin_rendering(&mut self) {
			// Hide cursor
			print!("\x1b[?25l")
		}

		fn end_rendering(&mut self) {
			// Show cursor
			print!("\x1b[?25h")
		}

		fn prepare_for_next_frame(&mut self, _frame: &Frame) {
			// Return cursor to "home position"
			print!("\x1b[1;1H")
		}

		fn render_frame(&mut self, frame: Frame) {
			let mut result = String::new();
			for item in frame {
				match item {
					FrameItem::Pixel(_x, _y, color) => {
						let r = (color.r * 255.) as i32;
						let g = (color.g * 255.) as i32;
						let b = (color.b * 255.) as i32;
						result.push_str(format!("\x1b[48;2;{r};{g};{b}m  ").as_str())
					}
					FrameItem::LineEnd => result.push_str("\x1b[0m\n"),
				}
			}
			println!("{}", result);
		}
	}
}
