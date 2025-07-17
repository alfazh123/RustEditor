use image::{GenericImageView};

pub fn get_size_handler(image_data: &[u8]) -> Vec<u32> {
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    let (width, height) = image.dimensions();
    return vec![width, height];
}