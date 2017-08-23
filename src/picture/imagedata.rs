pub struct ImageData {
	/* 	Represents a raw image data 
	 *	The type is u32 because this is the same size of a RGBA pixel, the default pixel size used
	 *	here.
	 */
	pub bdata: Vec<u32>
}

/* Pack bytes into a RHBA pixel */
pub fn pack_pixel(r: u32, g: u32, b: u32, a: u32) -> u32 
{
	(a) | (b << 8) | (g << 16) | (r << 24)
}

impl ImageData {
	/* Create an empty data */
	pub fn create_empty(size: usize) -> ImageData {
		let mut vbdata = Vec::with_capacity(size+1);
		vbdata.resize(size, 0);
		ImageData{bdata: vbdata}
	}

	/* 	Creates data from a pixel array
		Clones it so we maintain the reference */
	pub fn create_from_data(data: &[u32], size: usize) -> ImageData {
		let mut vbdata : Vec<u32> = Vec::with_capacity(size+1);
		
		let dd = data.clone();
		for &it in dd {
			vbdata.push(it);
		}

		vbdata.resize(size, 0);

		ImageData{bdata: vbdata}
	}

	pub fn set_data(&mut self, data: &[u32]) {
		let mut vbdata : Vec<u32> = Vec::with_capacity(self.bdata.len()+1);

		let dd = data.clone();
		for &it in dd {
			vbdata.push(it);
		}

		vbdata.resize(self.bdata.len(), 0);

		self.bdata = vbdata;
	}
}

/* Unit tests for imagedata */
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_imagedata_with_data() {
		let vtest = vec![1,2,3,4,5,6,7,8,9,1,2,3,4,5,6,7,8,9,1,2,3];

		let img = ImageData::create_from_data(&vtest[..], 8*8);
		
		assert_eq!(1, img.bdata[0]);
		assert_eq!(2, img.bdata[1]);
		assert_eq!(3, img.bdata[2]);
		assert_eq!(4, img.bdata[3]);
		assert_eq!(5, img.bdata[4]);
		assert_eq!(6, img.bdata[5]);

		assert_eq!(0x0, img.bdata[63], 
			"Wrong behavior: non-specified pixels should be transparent (0), they are {}", img.bdata[63]);
	}

	#[test]
	fn test_pack_pixel() {
		assert_eq!(0, pack_pixel(0, 0, 0, 0));
		assert_eq!(0xff000000, pack_pixel(255, 0, 0, 0));
		assert_eq!(0x00ff0000, pack_pixel(0, 255, 0, 0));
		assert_eq!(0x0000ff00, pack_pixel(0, 0, 255, 0));
		assert_eq!(0x000000ff, pack_pixel(0, 0, 0, 255));
	}

	#[test]
	fn test_imagedata_with_default_data() {
		let img = ImageData::create_empty(64);

		assert_eq!(0, img.bdata[0], 
			"Wrong behavior: non-specified pixels should be transparent (0), they are {}", img.bdata[0]);
	}

	#[test]
	fn test_imagedata_set() {
		let mut data = vec![0xff0000ff];
		data.resize(1024, 0xff0000ff);

		let mut img = ImageData::create_from_data(&data, 1024);

		assert_eq!(0xff0000ff, img.bdata[32]);

		let mut data = vec![0x0000ffff];
		data.resize(1024, 0x0000ffff);

		img.set_data(&data);

		assert_ne!(0xff0000ff, img.bdata[32], "Data not changed");
		assert_eq!(0x0000ffff, img.bdata[32], "Garbage in data");	
	}

}
