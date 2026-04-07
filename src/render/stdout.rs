use super::*;
static LIGHTNESS_CHARACTERS: [char; 10] = [' ', '.', ':', '-', '+', '=', '*', '@', '%', '#'];

pub struct StdoutAsciiRenderer;

impl RenderBackend for StdoutAsciiRenderer {
	fn begin_rendering(&mut self) {
		// Hide cursor
		print!("\x1b[?25l");
		// Return cursor to "home position"
		print!("\x1b[1;1H");
		// Enter alternative buffer
		print!("\x1b[?1049h");
	}

	fn end_rendering(&mut self) {
		// Show cursor
		print!("\x1b[?25h");
		// Leave alternate buffer
		print!("\x1b[?1049l");
	}

	fn prepare_for_next_frame(&mut self, _frame: &Frame) {
		// Return cursor to "home position"
		print!("\x1b[1;1H");
	}

	fn render_frame(&mut self, frame: Frame) {
		let mut result = String::new();
		for item in frame {
			match item {
				FrameItem::Pixel(_x, _y, color) => {
					let lightness = color.get_lightness();
					result.push(
						LIGHTNESS_CHARACTERS[(lightness * ((LIGHTNESS_CHARACTERS.len() - 1) as f64)
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

pub struct StdoutAnsiRenderer;

impl RenderBackend for StdoutAnsiRenderer {
	fn begin_rendering(&mut self) {
		// Hide cursor
		print!("\x1b[?25l");
		// Return cursor to "home position"
		print!("\x1b[1;1H");
		// Enter alternative buffer
		print!("\x1b[?1049h");
	}

	fn end_rendering(&mut self) {
		// Show cursor
		print!("\x1b[?25h");
		// Leave alternate buffer
		print!("\x1b[?1049l");
	}

	fn prepare_for_next_frame(&mut self, _frame: &Frame) {
		// Return cursor to "home position"
		print!("\x1b[1;1H");
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
