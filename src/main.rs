mod picture;

fn main() {
    let img = picture::Image::create_empty("Test", 256, 256);

    println!("{0} has {1}x{2} pixels", img.name, img.get_width(), img.get_height());
}
