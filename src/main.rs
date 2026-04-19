use engine::frame::Frame;
use engine::types::{Color, Coordinate2d, Triangle2d, Vector2};

use std::num::NonZeroU32;
use std::rc::Rc;

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::event::{StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop, OwnedDisplayHandle};
use winit::window::{Window, WindowId};

fn main() {
	let event_loop = EventLoop::new().unwrap();

	let context = Context::new(event_loop.owned_display_handle()).unwrap();
	let mut app = App {
		context,
		state: AppState::Initital,
	};

	event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
	event_loop.run_app(&mut app).unwrap();
}

struct App {
	context: Context<OwnedDisplayHandle>,
	state: AppState,
}

enum AppState {
	Initital,
	Suspended {
		window: Rc<Window>,
	},
	Running {
		surface: Surface<OwnedDisplayHandle, Rc<Window>>,
	},
}

impl ApplicationHandler for App {
	fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
		if let StartCause::Init = cause {
			let window_attrs = Window::default_attributes();
			let window = event_loop
				.create_window(window_attrs)
				.expect("Failed to create window");
			self.state = AppState::Suspended {
				window: Rc::new(window),
			}
		}
	}

	fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
		let AppState::Suspended { window } = &mut self.state else {
			unreachable!("Got resumed event while not suspended");
		};
		let mut surface =
			Surface::new(&self.context, window.clone()).expect("Failed to create surface");

		let size = window.inner_size();
		if let (Some(width), Some(height)) =
			(NonZeroU32::new(size.width), NonZeroU32::new(size.height))
		{
			surface.resize(width, height).unwrap();
		}

		self.state = AppState::Running { surface };
	}

	fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
		let AppState::Running { surface } = &mut self.state else {
			unreachable!("Got resumed event while not running");
		};
		let window = surface.window().clone();
		self.state = AppState::Suspended { window };
	}

	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		window_id: WindowId,
		event: WindowEvent,
	) {
		let AppState::Running { surface } = &mut self.state else {
			unreachable!("Got window event while suspended");
		};

		if surface.window().id() != window_id {
			return;
		}

		match event {
			WindowEvent::Resized(size) => {
				if let (Some(width), Some(height)) =
					(NonZeroU32::new(size.width), NonZeroU32::new(size.height))
				{
					surface.resize(width, height).unwrap();
				}
			}
			WindowEvent::RedrawRequested => {
				let size = surface.window().inner_size();

				let width = size.width as f64;
				let height = size.height as f64;

				let middle_x = width / 2.;
				let middle_y = height / 2.;

				let time = std::time::SystemTime::now()
					.duration_since(std::time::UNIX_EPOCH)
					.unwrap()
					.as_secs_f64() / 2.;

				let mut frame = Frame::new(size.width as usize, size.height as usize);

				let mut mini_frame = Frame::new(100, 100);
				mini_frame.callback_fill(|x, y| {
					if (y + x) / 8 % 2 == 0 {
						Color::white()
					} else {
						Color::transparent()
					}
				});

				frame.clear(Color::black());
				frame.draw_frame_int(Coordinate2d::one(), mini_frame);

				let tri = Triangle2d(
					Vector2::new(
						middle_x + f64::cos(time) * width / 20.,
						middle_y + f64::sin(time) * height / 20.,
					),
					Vector2::new(middle_x + f64::cos(time) * middle_x, middle_y),
					Vector2::new(middle_x, middle_y + f64::sin(time) * middle_y),
				);

				frame.draw_tri(tri, Color::green());

				frame.draw_wireframe_tri(tri, Color::red());

				let mut buffer = surface.buffer_mut().unwrap();
				let frame_data = frame.get_raw_data();
				for i in 0..frame_data.len() - 1 {
					buffer[i] = frame_data[i].get_compact_rgb();
				}

				buffer.present().unwrap();
				surface.window().request_redraw();
			}
			WindowEvent::CloseRequested => {
				event_loop.exit();
			}
			_ => (),
		}
	}
}
