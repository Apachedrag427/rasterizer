use super::*;
use base64::{Engine as _, engine::general_purpose};
use libc::{MAP_SHARED, O_CREAT, O_RDWR, PROT_WRITE, S_IRUSR, S_IWUSR, close, munmap};
use libc::{c_char, c_void, off_t};
use libc::{ftruncate, memcpy, mmap, shm_open};

const STORAGE_ID_STR: &'static str = "/KITTY_RASTERIZER_SHM";
const STORAGE_ID: *const c_char = b"/KITTY_RASTERIZER_SHM\0".as_ptr() as *const c_char;

pub struct KittySHMRenderer;

impl RenderBackend for KittySHMRenderer {
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
		// Delete all kitty images
		// println!("\x1b_Ga=d,d=A;\x1b\\");
	}

	fn render_frame(&mut self, frame: Frame) {
		let dim = frame.get_dimensions();
		let size_bytes = 3 * dim.0 * dim.1;
		// Initialize shared memory
		let (fd, addr) = unsafe {
			let null = std::ptr::null_mut();
			let fd = shm_open(STORAGE_ID, O_RDWR | O_CREAT, S_IRUSR | S_IWUSR);
			let _res = ftruncate(fd, size_bytes as off_t);
			let addr = mmap(null, size_bytes, PROT_WRITE, MAP_SHARED, fd, 0);

			(fd, addr)
		};

		let dimensions = frame.get_dimensions();
		let mut data: Vec<u8> = Vec::with_capacity(size_bytes);
		for item in frame {
			match item {
				FrameItem::Pixel(_x, _y, color) => {
					let r = (color.r * 255.) as u8;
					let g = (color.g * 255.) as u8;
					let b = (color.b * 255.) as u8;
					data.push(r);
					data.push(g);
					data.push(b);
				}
				FrameItem::LineEnd => (),
			}
		}
		let data = data;
		let pdata = data.as_ptr() as *const c_void;

		unsafe {
			memcpy(addr, pdata, data.len());
			munmap(addr, size_bytes);
			close(fd);
		}

		// It's the terminal's job to close the shm.
		print!(
			"\x1b_Ga=T,f=24,t=s,s={},v={};{}\x1b\\",
			dimensions.0,
			dimensions.1,
			general_purpose::STANDARD.encode(STORAGE_ID_STR)
		);
		std::io::Write::flush(&mut std::io::stdout()).unwrap();
	}
}

pub struct KittyEscapeRenderer;

impl RenderBackend for KittyEscapeRenderer {
	fn begin_rendering(&mut self) {
		// Hide cursor
		print!("\x1b[?25l");
		// Return cursor to "home position"
		print!("\x1b[1;1H");
	}

	fn end_rendering(&mut self) {
		// Show cursor
		print!("\x1b[?25h");
	}

	fn prepare_for_next_frame(&mut self, _frame: &Frame) {
		// Return cursor to "home position"
		print!("\x1b[1;1H");
		// Delete all kitty images
		println!("\x1bG_a=d,d=A;\x1b\\");
	}

	fn render_frame(&mut self, frame: Frame) {
		let dimensions = frame.get_dimensions();
		let mut data: Vec<u8> = Vec::new();
		for item in frame {
			match item {
				FrameItem::Pixel(_x, _y, color) => {
					let r = (color.r * 255.) as u8;
					let g = (color.g * 255.) as u8;
					let b = (color.b * 255.) as u8;
					data.push(r);
					data.push(g);
					data.push(b);
				}
				FrameItem::LineEnd => (),
			}
		}
		print!(
			"\x1b_Ga=T,f=24,s={},v={};{}\x1b\\",
			dimensions.0,
			dimensions.1,
			general_purpose::STANDARD.encode(&data[..])
		);
		std::io::Write::flush(&mut std::io::stdout()).unwrap();
	}
}
