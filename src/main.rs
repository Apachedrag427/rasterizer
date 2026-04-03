use rasterizer::{Color3, Frame};

pub mod renderers;

use renderers::Renderer;

fn main() {
	let width = 200;
	let height = 100;

	let mut renderer = renderers::stdout_grayscale_ascii_renderer::StdoutGrayscaleAsciiRenderer;
	renderer.begin_rendering();

	let mut offset = 0f64;

	loop {
		let mut frame = Frame::new(width, height);

		frame.callback_fill(|x, y| {
			let x_prog = (x as f64) / (width - 1) as f64;
			let y_prog = (y as f64) / (height - 1) as f64;

			let value = f64::sin(offset + x_prog + y_prog) * 0.5 + 0.5;

			Color3::new(value, value, value)
		});

		offset += 0.005;

		renderer.prepare_for_next_frame(&frame);
		renderer.render_frame(frame);

		std::thread::sleep(std::time::Duration::from_millis(10));
	}
}
