use std::io::Cursor;
use image::{ImageBuffer, Rgb};
use colors_transform::{Rgb as ColorTransformRgb, Hsl, Color}; // Import trait ColorTransform
use crate::lab_converter::{lab_to_rgb, rgb_to_lab};

pub fn adjust_temperature_handler(image_data: &[u8], temperature: f64) -> Vec<u8> {
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    let rgb_image = image.to_rgb8();
    let (width, height) = rgb_image.dimensions();
    let mut adjusted_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    let temperature_value: f64;

    if temperature > 0.0 {
        if temperature > 6.0 {
            temperature_value = 18.0;
        } else {
            temperature_value = (temperature * 2.0) + temperature;
        }
    } else if temperature == 0.0 {
        temperature_value = 0.0;
    } else {
        if temperature < -6.0 {
            temperature_value = -18.0;
        } else {
            temperature_value = (temperature * 2.0) + temperature;
        }
    }

    for (x, y, pixel) in rgb_image.enumerate_pixels() {
        let adjusted_pixel = adjust_temperature(*pixel, temperature_value);
        adjusted_image.put_pixel(x, y, adjusted_pixel);
    }

    let mut buf = Cursor::new(Vec::new());
    adjusted_image.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write the image");
    buf.into_inner()
}

fn adjust_temperature(
    pixel: Rgb<u8>,
    temperature_factor: f64,
) -> Rgb<u8> {
    let lab = rgb_to_lab(pixel);
    let mut adjusted_lab = lab;
    adjusted_lab[2] += temperature_factor * 1.5; // Adjust the blue-yellow axis
    let new_rgb = lab_to_rgb(adjusted_lab);
    Rgb([
        new_rgb[0] as u8,
        new_rgb[1] as u8,
        new_rgb[2] as u8,
    ])
}

pub fn adjust_tint_handler(image_data: &[u8], tint: f64) -> Vec<u8> {
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    let rgb_image = image.to_rgb8();
    let (width, height) = rgb_image.dimensions();
    let mut adjusted_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    let tint_value: f64;

    if tint > 0.0 {
        if tint > 6.0 {
            tint_value = 18.0;
        } else {
            tint_value = (tint * 2.0) + tint;
        }
    } else if tint == 0.0 {
        tint_value = 0.0;
    } else {
        if tint < -6.0 {
            tint_value = -18.0;
        } else {
            tint_value = (tint * 2.0) + tint;
        }
    }

    for (x, y, pixel) in rgb_image.enumerate_pixels() {
        let adjusted_pixel = adjust_tint(*pixel, tint_value);
        adjusted_image.put_pixel(x, y, adjusted_pixel);
    }

    let mut buf = Cursor::new(Vec::new());
    adjusted_image.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write the image");
    buf.into_inner()
}

fn adjust_tint(
    pixel: Rgb<u8>,
    tint_factor: f64,
) -> Rgb<u8> {
    let lab = rgb_to_lab(pixel);
    let mut adjusted_lab = lab;
    adjusted_lab[1] += tint_factor * 1.5; // Adjust the blue-yellow axis
    let new_rgb = lab_to_rgb(adjusted_lab);
    Rgb([
        new_rgb[0] as u8,
        new_rgb[1] as u8,
        new_rgb[2] as u8,
    ])
}

pub fn adjust_saturation_handler(image_data: &[u8], saturation: f32) -> Vec<u8> {
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    let rgb_image = image.to_rgb8();
    let (width, height) = rgb_image.dimensions();
    let mut adjusted_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    let saturation_value: f32;

    if saturation > 0.0 {
        if saturation > 6.0 {
            saturation_value = 1.0 + (6.0/6.0);
        } else {
            saturation_value = 1.0 + (saturation/6.0);
        }
    } else if saturation == 0.0 {
        saturation_value = 1.0;
    } else {
        if saturation < -6.0 {
            saturation_value = 0.0;
        } else {
            saturation_value = (saturation + 6.0) / 6.0;
        }
    }

    for (x, y, pixel) in rgb_image.enumerate_pixels() {
        let adjusted_pixel = adjust_saturation(*pixel, saturation_value);
        adjusted_image.put_pixel(x, y, adjusted_pixel);
    }

    let mut buf = Cursor::new(Vec::new());
    adjusted_image.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write the image");
    buf.into_inner()
}

fn adjust_saturation(
    pixel: Rgb<u8>,
    saturation_factor: f32,
) -> Rgb<u8> {
    let rgb = ColorTransformRgb::from(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32);
    let hsl = rgb.to_hsl();
    println!("HSL: {:?}", hsl.get_saturation());
    let adjusted_hsl = Hsl::from(hsl.get_hue(), hsl.get_saturation() * saturation_factor, hsl.get_lightness());
    //     h: hsl.h,
    //     s: (hsl.s * saturation_factor).clamp(0.0, 1.0), // Adjust saturation
    //     l: hsl.l,
    // };
    let adjusted_rgb = adjusted_hsl.to_rgb();
    Rgb([
        adjusted_rgb.get_red() as u8,
        adjusted_rgb.get_green() as u8,
        adjusted_rgb.get_blue() as u8,
    ])
}