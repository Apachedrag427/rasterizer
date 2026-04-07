use rasterizer::frame::Frame;
use rasterizer::types::{Color3, Rect, Vector2};

use rasterizer::render;
use rasterizer::render::RenderBackend;

fn main() {
	let width = 800;
	let height = 800;

	let middle_x = width as f64 / 2.;
	let middle_y = height as f64 / 2.;

	let mut renderer = render::KittySHMRenderer;
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

		let arrow_length = 300.;
		let arrow_leaf_length = 50.;
		let arrow_leaf_offset = 0.5;

		let arrow_color = bg_color.invert(); //Color3::new(f64::sin(time) * 0.5 + 0.5, f64::cos(time) * 0.5 + 0.5, 1.);

		let arrow_point = Vector2::new(
			middle_x + f64::cos(time) * arrow_length,
			middle_y + f64::sin(time) * arrow_length,
		);

		frame.draw_line(Vector2::new(middle_x, middle_y), arrow_point, arrow_color);
		frame.draw_line(
			arrow_point,
			Vector2::new(
				arrow_point.x - arrow_leaf_length * f64::cos(time + arrow_leaf_offset),
				arrow_point.y - arrow_leaf_length * f64::sin(time + arrow_leaf_offset),
			),
			arrow_color,
		);
		frame.draw_line(
			arrow_point,
			Vector2::new(
				arrow_point.x - arrow_leaf_length * f64::cos(time - arrow_leaf_offset),
				arrow_point.y - arrow_leaf_length * f64::sin(time - arrow_leaf_offset),
			),
			arrow_color,
		);

		frame.fill_rect(
			Rect {
				position: Vector2::new(150., 150.),
				dimensions: Vector2::new(100., 1.),
			},
			Color3::black(),
		);

		renderer.prepare_for_next_frame(&frame);
		renderer.render_frame(frame);

		std::thread::sleep(std::time::Duration::from_millis(10));
	}
}
