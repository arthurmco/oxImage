/* Basic image file opener */
extern crate image;
use self::image::{GenericImage, Pixel};

use super::Image;
use super::imagedata;

use std::path::Path;
pub use self::image::ImageError;

pub struct BasicImageOpener {
	filename: String,
}

impl BasicImageOpener {
	pub fn open(filename: &str) -> Result<Image, image::ImageError> {
		let img = image::open(&Path::new(filename))?;
		let (w, h) = img.dimensions();

		let mut data = Vec::<u32>::with_capacity((w*h+1) as usize);
		data.resize((w*h) as usize, 0);

		/* Plot pixels in the data array */
		for pixel in img.pixels() {
			let (x, y, val) = pixel;
			let loffset: usize = (y*w+x) as usize;
			let val = val.to_rgba();
			let cr = val.data[0] as u32;
			let cg = val.data[1] as u32;
			let cb = val.data[2] as u32;
			let ca = val.data[3] as u32;

			data[loffset] = imagedata::pack_pixel(cr, cg, cb, ca);
		}

		return Ok(Image::create_from_data(&filename, &filename, w, h, &data));

	}
}

/* Tests for Imagefile class */
#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn open_image_all_black() {
		let image = BasicImageOpener::open("tests/testimg0.png")
			.expect("Could not open file");
		
		assert_eq!(640, image.get_width());
		assert_eq!(400, image.get_height());
		assert_eq!(0x000000ff, image.get_pixel(0, 0));
		assert_eq!(0x000000ff, image.get_pixel(100, 0));
		assert_eq!(0x000000ff, image.get_pixel(100, 100));
		assert_eq!(0x000000ff, image.get_pixel(200, 100));
		assert_eq!(0x000000ff, image.get_pixel(200, 200));
		assert_eq!(0x000000ff, image.get_pixel(300, 100));
	}

	#[test]
	fn open_image_all_red() {
		let image = BasicImageOpener::open("tests/testimg1.png")
			.expect("Could not open file");
		
		assert_eq!(1280, image.get_width());
		assert_eq!(820, image.get_height());
		assert_eq!(0xff0000ff, image.get_pixel(0, 0));
		assert_eq!(0xff0000ff, image.get_pixel(100, 0));
		assert_eq!(0xff0000ff, image.get_pixel(300, 100));
		assert_eq!(0xff0000ff, image.get_pixel(500, 100));
		assert_eq!(0xff0000ff, image.get_pixel(600, 201));
		assert_eq!(0xff0000ff, image.get_pixel(800, 100));
	}
}

