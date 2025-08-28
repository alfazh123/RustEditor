use std::io::Cursor;
use image::{ImageBuffer, Rgb};
use crate::lab_converter::{lab_to_rgb, rgb_to_lab};

pub fn adjust_exposure_handler(image_data: &[u8], exposure: f64) -> Vec<u8> {
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    let rgb_image = image.to_rgb8();
    let (width, height) = rgb_image.dimensions();
    let mut adjusted_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    let exposure_value: f64;

    if exposure > 0.0 {
        if exposure > 6.0 {
            exposure_value = 20.0;
        } else {
            exposure_value = (exposure * 3.0) + 3.0;
        }
    } else if exposure == 0.0 {
        exposure_value = 0.0;
    } else {
        if exposure == -6.0 || exposure < -6.0 {
            exposure_value = -20.0;
        } else {
            exposure_value = (exposure * 3.0) + 3.0;
        }
    }

    for (x, y, pixel) in rgb_image.enumerate_pixels() {
        let adjusted_pixel = adjust_exposure(*pixel, exposure_value);
        adjusted_image.put_pixel(x, y, adjusted_pixel);
    }

    let mut buf = Cursor::new(Vec::new());
    adjusted_image.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write the image");
    buf.into_inner()
}

fn adjust_exposure(
    pixel: Rgb<u8>,
    exposure_factor: f64,
) -> Rgb<u8> {
    let lab = rgb_to_lab(pixel);
    let new_lab = [
        (lab[0] + exposure_factor).clamp(0.0, 100.0), // Lightness channel
        lab[1], 
        lab[2], 
    ];
    let new_rgb = lab_to_rgb(new_lab);
    Rgb([
        new_rgb[0] as u8,
        new_rgb[1] as u8,
        new_rgb[2] as u8,
    ])
}

pub fn adjust_contrasts_handler(image_data: &[u8], contrasts: f64) -> Vec<u8> {
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    let rgb_image = image.to_rgb8();
    let (width, height) = rgb_image.dimensions();
    let mut adjusted_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    let contrasts_value: f64;

    if contrasts > 0.0 {
        if contrasts > 6.0 {
            contrasts_value = (6.0 + 6.0) / 6.0;
        } else {
            contrasts_value = (contrasts + 6.0) / 6.0;
        }
    } else if contrasts == 0.0 {
        contrasts_value = 1.0;
    } else {
        if contrasts < -6.0 {
            contrasts_value = (12.0 + (-6.0)) / 12.0;
        } else {
            contrasts_value = (12.0 + contrasts) / 12.0;
        }
    }

    for (x, y, pixel) in rgb_image.enumerate_pixels() {
        let adjusted_pixel = adjust_contrast(*pixel, contrasts_value);
        adjusted_image.put_pixel(x, y, adjusted_pixel);
    }

    let mut buf = Cursor::new(Vec::new());
    adjusted_image.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write the image");
    buf.into_inner()
}

fn adjust_contrast(pixel: Rgb<u8>, contrast_factor: f64) -> Rgb<u8> {
    let r = pixel[0];
    let g = pixel[1];
    let b = pixel[2];
    
    // Apply contrast adjustment: new_value = (old_value - 128) * factor + 128
    // 128 is the middle gray point (0.5 * 255)
    let adjusted_r = ((r as f64 - 128.0) * contrast_factor + 128.0).clamp(0.0, 255.0);
    let adjusted_g = ((g as f64 - 128.0) * contrast_factor + 128.0).clamp(0.0, 255.0);
    let adjusted_b = ((b as f64 - 128.0) * contrast_factor + 128.0).clamp(0.0, 255.0);
    Rgb([
        adjusted_r as u8,
        adjusted_g as u8,
        adjusted_b as u8,
    ])
}

pub fn adjust_ligth(image_data: &[u8], contrast: f64, exposure: f64) -> Vec<u8> {

    let contrasts_value: f64 = match contrast {
        c if c > 6.0 => 2.0,
        c if c > 0.0 => (c + 6.0) / 6.0,
        c if c == 0.0 => 1.0,
        c if c < -6.0 => 0.5,
        c if c < 0.0 => (12.0 + c) / 12.0,
        _ => unreachable!()
    };

    let exposure_value: f64 = match exposure {
        e if e > 6.0 => 20.0,
        e if e > 0.0 => (e * 3.0) + 3.0,
        e if e == 0.0 => 0.0,
        e if e < -6.0 => -20.0,
        e if e < 0.0 => (e * 3.0) + 3.0,
        _ => unreachable!()
    };
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    let rgb_image = image.to_rgb8();
    let (width, height) = rgb_image.dimensions();
    let mut adjusted_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);


    for (x, y, pixel) in rgb_image.enumerate_pixels() {
        let exposure_adjusted = adjust_exposure(*pixel, exposure_value);
        let contrast_adjusted = adjust_contrast(exposure_adjusted, contrasts_value);
        adjusted_image.put_pixel(x, y, contrast_adjusted);
    }


    let mut buf = Cursor::new(Vec::new());
    adjusted_image.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write the image");
    buf.into_inner()
}