use std::collections::HashMap;
use super::{ImageCollectionElement, ImageCollectionItem, Image};

pub struct ImageCollectionList<> {
	/*	Represents an image collection */
	pub name: String,

	items: HashMap<String, ImageCollectionItem>	
}

impl ImageCollectionElement for ImageCollectionList {
	fn get_display_name(&self) -> String { self.name.clone() }
	fn get_file_name(&self) -> String { self.name.clone() }
}

impl ImageCollectionList {
	pub fn create(name: &str) -> ImageCollectionList {
		ImageCollectionList{items: HashMap::new(), name: String::from(name)}
	}

	pub fn add_element(&mut self, el: ImageCollectionItem) {
		let fname = match el {
			ImageCollectionItem::ImageItem(ref im) => im.get_file_name(),
			ImageCollectionItem::ImageList(ref l) => l.get_file_name(),
		};

		println!("Adding {}", fname);

		self.items.insert(fname, el);	
	}

	pub fn get_element(&self, filename: &str) -> Option<&ImageCollectionItem> {
		self.items.get(filename)
	}

	pub fn rename_element(&mut self, oldname: &str, newname: &str) {
		panic!("Not implemented yet");
	}

	pub fn remove_element(&mut self, filename: &str) {
		self.items.remove(filename);
	}

	pub fn next(&mut self) -> Option<&ImageCollectionItem> {
		self.items.values().next()
	}

	pub fn count(&self) -> usize {
		self.items.len()
	}

	
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::ImageCollectionItem::ImageItem;

	#[test]
	fn test_image_collection_item_add() {
		let mut ic = ImageCollectionList::create("list");

		let im1 = Image::create_empty("img1", 16, 16);
		let im2 = Image::create_empty("img2", 20, 20);

		ic.add_element(ImageItem(im1));
		ic.add_element(ImageItem(im2));

		assert_eq!(2, ic.count());
	}

	#[test]
	fn test_image_collection_item_get() {
		let mut ic = ImageCollectionList::create("list");

		let im1 = Image::create_empty("img1", 16, 16);
		let im2 = Image::create_empty("img2", 20, 20);
		let im1path = im1.path.clone();

		ic.add_element(ImageItem(im1));
		ic.add_element(ImageItem(im2));

		let iim1 = if let Some(iopt) = ic.get_element(&im1path) {
			if let ImageItem(ref it) = *iopt {
				it
			} else {
				panic!("Incorrect type returned from get_element()");
			}
		} else {
			panic!("There's no value, expected a value here ");
		};

		assert_eq!(16, iim1.get_width());
		assert_eq!(im1path, iim1.path);
	}

	#[test]
	fn test_image_collection_remove() {
		let mut ic = ImageCollectionList::create("list");

		let im1 = Image::create_empty("img1", 16, 16);
		let im2 = Image::create_empty("img2", 20, 20);
		let im1path = im1.path.clone();
		let im2path = im2.path.clone();

		ic.add_element(ImageItem(im1));
		ic.add_element(ImageItem(im2));
		assert_eq!(true, ic.get_element(&im1path).is_some());

		ic.remove_element(&im1path);
		assert_eq!(true, ic.get_element(&im1path).is_none());
		assert_eq!(true, ic.get_element(&im2path).is_some());
	}
}

