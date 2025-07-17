use wasm_bindgen::prelude::*;
use std::{io::Cursor};
use image::{GenericImageView};
mod switch_color;
mod image_size;
mod color;
mod light;
mod lab_converter;
use switch_color::{switch_color_handler};
use image_size::{get_size_handler};
use color::{adjust_temperature_handler, adjust_tint_handler, adjust_saturation_handler};
use light::{adjust_exposure_handler, adjust_contrasts_handler};

#[wasm_bindgen]
pub fn get_size(image_data: &[u8]) -> Vec<u32> {
    get_size_handler(image_data)
}

#[wasm_bindgen]
pub fn switch_color(image_source: &[u8], image_reference: &[u8]) -> Vec<u8> {
    switch_color_handler(image_source, image_reference)
}

#[wasm_bindgen]
pub fn blur(image_data: &[u8], radius: f32) -> Vec<u8> {
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    // let image = image::load_from_memory(fix_size_image(image_data).as_slice()).expect("Failed to open the file");
    let blurred_image = image.blur(radius);
    let mut buf = Cursor::new(Vec::new());
    blurred_image.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write the image");
    buf.into_inner()
}

#[wasm_bindgen]
pub fn sharpen(image_data: &[u8], radius: f32) -> Vec<u8> {
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    let sharpened_image = image.unsharpen(radius, 1);
    let mut buf = Cursor::new(Vec::new());
    sharpened_image.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write the image");
    buf.into_inner()
}

#[wasm_bindgen]
pub fn fix_size_image(image_data: &[u8]) -> Vec<u8> {
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    let (width, height) = image.dimensions();
    if width > 1267 {
        let resize_image = image.resize(1267, height, image::imageops::FilterType::Lanczos3);
        let mut buf = Cursor::new(Vec::new());
        resize_image.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write the image");
        buf.into_inner()
    } else if height > 1267 {
        let resize_image = image.resize(width, 1267, image::imageops::FilterType::Lanczos3);
        let mut buf = Cursor::new(Vec::new());
        resize_image.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write the image");
        buf.into_inner()
    } else {
        let mut buf = Cursor::new(Vec::new());
        image.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write the image");
        buf.into_inner()
    }
}

#[wasm_bindgen]
pub fn grayscale_image(image_data: &[u8]) -> Vec<u8> {
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    let gray_image = image.to_luma8();
    let mut buf = Cursor::new(Vec::new());
    gray_image.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write the image");
    buf.into_inner()
}

#[wasm_bindgen]
pub fn adjust_saturation_image(image_data: &[u8], saturation: f32) -> Vec<u8> {
    adjust_saturation_handler(image_data, saturation)
}

#[wasm_bindgen]
pub fn adjust_temperature_image(image_data: &[u8], temperature: f64) -> Vec<u8> {
    adjust_temperature_handler(image_data, temperature)
}

#[wasm_bindgen]
pub fn adjust_tint_image(image_data: &[u8], tint: f64) -> Vec<u8> {
    adjust_tint_handler(image_data, tint)
}

#[wasm_bindgen]
pub fn adjust_exposure_image(image_data: &[u8], exposure: f64) -> Vec<u8> {
    adjust_exposure_handler(image_data, exposure)
}


#[wasm_bindgen]
pub fn adjust_contrasts_image(image_data: &[u8], contrasts: f32) -> Vec<u8> {
    adjust_contrasts_handler(image_data, contrasts)
}