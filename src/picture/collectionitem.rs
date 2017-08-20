use super::Image;
use super::ImageCollectionList;

pub enum ImageCollectionItem {
	/* 	Represents an image collection item
	 *	It can be an image or an collection of images
	 */

	ImageItem(Image),
	ImageList(ImageCollectionList),
}

