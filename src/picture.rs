pub struct ImageData {
	/* Represents a raw image data */
	bdata: Vec<u32>
}

impl ImageData {
	/* Create an empty data */
	pub fn create_empty() -> ImageData {
		ImageData{bdata: Vec::new()}
	}

	/* 	Creates data from a pixel array
		Clones it so we maintain the reference */
	pub fn create_from_data(data: &[u32]) -> ImageData {
		let mut vbdata : Vec<u32> = Vec::new();
		
		let dd = data.clone();
		for &it in dd {
			vbdata.push(it);
		}

		ImageData{bdata: vbdata}
	}
}

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
			path: String::default(), data: ImageData::create_empty() }
	}

	pub fn create_from_data(name: &str, file: &str, width: u32, height: u32, data: &[u32]) -> Image {
		Image{path: String::from(file), name: String::from(name), width: width, 
			height: height, data: ImageData::create_from_data(data)}
	}

	pub fn get_width(&self) -> u32 { self.width }
	pub fn get_height(&self) -> u32 { self.height }

}
