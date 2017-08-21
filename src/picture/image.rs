use super::imagedata::ImageData;
use super::ImageCollectionElement;

pub struct Image {
	/* 	Represents an image inside the software, already decoded. 
		Since all decoded images are always RGBA, there's no need of specifying the format
	*/
	pub path: String,
	pub name: String,
	width: u32,
	height: u32,
	data: ImageData,
}

impl ImageCollectionElement for Image {
	fn get_display_name(&self) -> String { self.name.clone() }
	fn get_file_name(&self) -> String { self.path.clone() }
}

impl Image {
	pub fn create_empty(name: &str, width: u32, height: u32) -> Image {
		let mut lpath = String::from("@");
		lpath.push_str(name);

		Image{name: String::from(name), width: width, height: height, 
			path: lpath, data: ImageData::create_empty((width*height) as usize) }
	}

	pub fn create_from_data(name: &str, file: &str, width: u32, height: u32, data: &[u32]) -> Image {
		Image{path: String::from(file), name: String::from(name), width: width, 
			height: height, data: ImageData::create_from_data(data, (width*height) as usize)}
	}

	pub fn resize_area(&mut self, nwidth: u32, nheight: u32) {
		/* Resize the image area. It won't resiz any data */
		let mut idata = ImageData::create_empty((nwidth*nheight) as usize);
		let mut x = 0;
		let mut y = 0;

		for pixel in &self.data.bdata {
			
			if y < self.height {
				idata.bdata[(y*nwidth + x) as usize] = *pixel;
				x += 1;
			}
			
			if x == self.width {
				while x < nwidth {
					x += 1;
				}
				
				y += 1;
				x = 0;
			}
		}

		self.data = idata;		
	}
	

	pub fn get_width(&self) -> u32 { self.width }
	pub fn get_height(&self) -> u32 { self.height }

}

/* Tests for Image class */
#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_parameters() {
		let i = Image::create_empty("image_test", 800, 600);
		
		assert_eq!(i.get_width(), 800);
		assert_eq!(i.get_height(), 600);
		assert_eq!(i.name, "image_test");

	}

	#[test]
	fn test_resize_area() {
		let mut data = vec![0xff0000ff];
		data.resize(1024, 0xff0000ff);

		let mut i = Image::create_from_data("name_test", "name_test", 32, 32, &data);
		assert_eq!(0xff0000ff, i.data.bdata[48], "Not correctly loaded");

		i.resize_area(64, 64);
		assert_ne!(0xff0000ff, i.data.bdata[48], "Not correctly resized: not even resized");
		assert_eq!(0x0, i.data.bdata[48], "Not correctly resized (garbage)");
	}
}
