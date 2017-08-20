pub struct ImageData {
	/* Represents a raw image data */
	pub bdata: Vec<u32>
}

impl ImageData {
	/* Create an empty data */
	pub fn create_empty(size: usize) -> ImageData {
		ImageData{bdata: Vec::with_capacity(size)}
	}

	/* 	Creates data from a pixel array
		Clones it so we maintain the reference */
	pub fn create_from_data(data: &[u32], size: usize) -> ImageData {
		let mut vbdata : Vec<u32> = Vec::with_capacity(size+1);
		
		let dd = data.clone();
		for &it in dd {
			vbdata.push(it);
		}

		ImageData{bdata: vbdata}
	}

	pub fn set_data(&mut self, data: &[u32]) {
		let mut vbdata : Vec<u32> = Vec::with_capacity(self.bdata.len()+1);
		
		let dd = data.clone();
		for &it in dd {
			vbdata.push(it);
		}

		self.bdata = vbdata;
	}
}
