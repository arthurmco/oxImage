pub trait ImageCollectionElement {
	/* 	Trait for image collections elements
	 *	Every item that needs to be in an image collection should implement this
	 */

	 fn get_display_name(&self) -> String;
	 fn get_file_name(&self) -> String;
}

