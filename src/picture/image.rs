use super::imagedata::ImageData;

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

impl Image {
	pub fn create_empty(name: &str, width: u32, height: u32) -> Image {
		Image{name: String::from(name), width: width, height: height, 
			path: String::default(), data: ImageData::create_empty((width*height) as usize) }
	}

	pub fn create_from_data(name: &str, file: &str, width: u32, height: u32, data: &[u32]) -> Image {
		Image{path: String::from(file), name: String::from(name), width: width, 
			height: height, data: ImageData::create_from_data(data, (width*height) as usize)}
	}

	pub fn resize(&self, nwidth: u32, nheight: u32) {
		let mut idata = ImageData::create_empty((nwidth*nheight) as usize);
		let mut x = 0;
		let mut y = 0;

		for pixel in &self.data.bdata {
			if y < self.height {
				idata.bdata.push(*pixel);
				x += 1;
			}

			if x == self.width {
				while x < nwidth {
					idata.bdata.push(0);
					x += 1;
				}
				y += 1;
				x = 0;
			}
		}
		
	}

	pub fn get_width(&self) -> u32 { self.width }
	pub fn get_height(&self) -> u32 { self.height }

}
