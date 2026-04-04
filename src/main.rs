use rasterizer::{Color3, Frame, Vector2};

pub mod renderers;

use renderers::Renderer;

fn main() {
	let width = 100;
	let height = 100;

	let middle_x = width as f64 / 2.;
	let middle_y = height as f64 / 2.;

	let mut renderer = renderers::stdout_color_ansi_renderer::StdoutColorAnsiRenderer;
	renderer.begin_rendering();

	loop {
		let time = std::time::SystemTime::now()
			.duration_since(std::time::UNIX_EPOCH)
			.unwrap()
			.as_secs_f64();
		let mut frame = Frame::new(width, height);

		let bg_color =
			Color3::new(f64::cos(time) * 0.5 + 0.5, f64::sin(time) * 0.5 + 0.5, 0.) * 0.65;

		frame.clear(bg_color);

		let arrow_color = bg_color.invert(); //Color3::new(f64::sin(time) * 0.5 + 0.5, f64::cos(time) * 0.5 + 0.5, 1.);

		let arrow_point = Vector2::new(
			middle_x + f64::cos(time) * 40.,
			middle_y + f64::sin(time) * 40.,
		);

		frame.draw_line(Vector2::new(middle_x, middle_y), arrow_point, arrow_color);
		frame.draw_line(
			arrow_point,
			Vector2::new(
				arrow_point.x - 7. * f64::cos(time + 0.5),
				arrow_point.y - 7. * f64::sin(time + 0.5),
			),
			arrow_color,
		);
		frame.draw_line(
			arrow_point,
			Vector2::new(
				arrow_point.x - 7. * f64::cos(time - 0.5),
				arrow_point.y - 7. * f64::sin(time - 0.5),
			),
			arrow_color,
		);

		renderer.prepare_for_next_frame(&frame);
		renderer.render_frame(frame);

		std::thread::sleep(std::time::Duration::from_millis(10));
	}
}
