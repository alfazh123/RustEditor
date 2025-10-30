use std::{io::Cursor};
use image::{ImageBuffer, Rgb};
use crate::lab_converter::{lab_to_rgb, rgb_to_lab};
// use colors_transform::{Rgb as ColorTransformRgb, Hsl, Color}; // Import trait ColorTransform

pub fn adjust_temperature_handler(image_data: &[u8], temperature: f64) -> Vec<u8> {
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    let rgb_image = image.to_rgb8();
    let (width, height) = rgb_image.dimensions();
    let mut adjusted_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    let temperature_value: f64;

    if temperature > 0.0 {
        if temperature > 6.0 {
            temperature_value = 24.0;
            // temperature_value = 100.0;
        } else {
            temperature_value = temperature * 4.0;
            // temperature_value = temperature / 6.0 * 100.0;
        }
    } else if temperature == 0.0 {
        temperature_value = 0.0;
    } else {
        if temperature < -6.0 {
            temperature_value = -24.0;
            // temperature_value = -100.0;
        } else {
            temperature_value = temperature * 4.0;
            // temperature_value = temperature / 6.0 * 100.0;
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
    // let lab = rgb_to_lab(pixel);
    // let mut adjusted_lab = [lab[0], lab[1], lab[2]];
    // adjusted_lab[2] += temperature_factor * 1.5; // Adjust the blue-yellow axis
    // let new_rgb = lab_to_rgb(adjusted_lab);
    // Rgb([
    //     new_rgb[0] as u8,
    //     new_rgb[1] as u8,
    //     new_rgb[2] as u8,
    // ])
    let r = pixel.0[0] as f64;
    let g = pixel.0[1] as f64;
    let b = pixel.0[2] as f64;

    let new_r = r + temperature_factor;
    let new_b = b - temperature_factor;

    let adjusted_rgb = Rgb([
        new_r.clamp(0.0, 255.0) as u8,
        g.clamp(0.0, 255.0) as u8,
        new_b.clamp(0.0, 255.0) as u8,
    ]);

    Rgb([
        adjusted_rgb[0],
        adjusted_rgb[1],
        adjusted_rgb[2],
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
            tint_value = 24.0;
            // tint_value = 100.0;
        } else {
            tint_value = tint * 4.0;
            // tint_value = tint / 6.0 * 100.0;
        }
    } else if tint == 0.0 {
        tint_value = 0.0;
    } else {
        if tint < -6.0 {
            tint_value = -24.0;
            // tint_value = -100.0;
        } else {
            tint_value = tint * 4.0;
            // tint_value = tint / 6.0 * 100.0;
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
    // let lab = rgb_to_lab(pixel);
    // let mut adjusted_lab = lab;
    // adjusted_lab[1] += tint_factor * 1.5; // Adjust the blue-yellow axis
    // let new_rgb = lab_to_rgb(adjusted_lab);
    // Rgb([
    //     new_rgb[0] as u8,
    //     new_rgb[1] as u8,
    //     new_rgb[2] as u8,
    // ])
    let r = pixel.0[0] as f64;
    let g = pixel.0[1] as f64;
    let b = pixel.0[2] as f64;

    let new_g = g + tint_factor;

    let adjusted_rgb = Rgb([
        r.clamp(0.0, 255.0) as u8,
        new_g.clamp(0.0, 255.0) as u8,
        b.clamp(0.0, 255.0) as u8,
    ]);

    Rgb([
        adjusted_rgb[0],
        adjusted_rgb[1],
        adjusted_rgb[2],
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
    // Convert to LAB for more accurate saturation adjustment
    let r = pixel.0[0] as f32;
    let g = pixel.0[1] as f32;
    let b = pixel.0[2] as f32;
    
    // Calculate grayscale using fast luminance approximation
    let gray = r * 0.299 + g * 0.587 + b * 0.114;
    
    // Linear interpolation between gray and original color
    let new_r = gray + (r - gray) * saturation_factor;
    let new_g = gray + (g - gray) * saturation_factor;
    let new_b = gray + (b - gray) * saturation_factor;
    
    let adjusted_pixel = Rgb([
        new_r.clamp(0.0, 255.0) as u8,
        new_g.clamp(0.0, 255.0) as u8,
        new_b.clamp(0.0, 255.0) as u8,
    ]);

    // let new_rgb = lab_to_rgb(adjusted_lab);
    Rgb([
        // new_rgb[0],
        // new_rgb[1],
        // new_rgb[2],
        adjusted_pixel[0],
        adjusted_pixel[1],
        adjusted_pixel[2],
    ])
}

pub fn adjust_color_handler(image_data: &[u8], saturation_value: f64, temperature_value: f64, tint_value: f64) -> Vec<u8> {
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    let rgb_image = image.to_rgb8();
    let (width, height) = rgb_image.dimensions();
    let mut adjusted_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    let saturation: f64 = match saturation_value {
        s if s > 6.0 => 2.0, 
        s if s > 0.0 => 1.0 + (s / 6.0),
        0.0 => 1.0,
        s if s < -6.0 => 0.0,
        s if s < 0.0 => (s + 6.0) / 6.0,
        _ => unreachable!(), 
    };

    let tint: f64 = match tint_value {
        t if t > 6.0 => 18.0,
        t if t > 0.0 => t * 3.0, 
        0.0 => 0.0,
        t if t < -6.0 => -18.0,
        t if t < 0.0 => t * 3.0, 
        _ => unreachable!(),
    };

    let temperature: f64 = match temperature_value {
        t if t > 6.0 => 18.0,
        t if t > 0.0 => t * 3.0, 
        0.0 => 0.0,
        t if t < -6.0 => -18.0,
        t if t < 0.0 => t * 3.0, 
        _ => unreachable!(),
    };

    for (x, y, pixel) in rgb_image.enumerate_pixels() {
        let adjusted_pixel = adjust_color(*pixel, saturation, temperature, tint);
        adjusted_image.put_pixel(x, y, adjusted_pixel);
    }

    let mut buf = Cursor::new(Vec::new());
    adjusted_image.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write the image");
    buf.into_inner()
}

fn adjust_color(
    pixel: Rgb<u8>,
    saturation: f64,
    temperature: f64,
    tint: f64
) -> Rgb<u8> {
    // Single RGB->LAB conversion
    let lab = rgb_to_lab(pixel);
    let mut adjusted_lab = lab;
    
    // Apply temperature and tint directly in LAB space
    adjusted_lab[1] += tint * 1.5;      
    adjusted_lab[2] += temperature * 1.5; 
    
    if saturation != 1.0 {
        let original_chroma = (adjusted_lab[1].powi(2) + adjusted_lab[2].powi(2)).sqrt();
        if original_chroma > 0.0 {
            let saturation_factor = saturation;
            adjusted_lab[1] *= saturation_factor;
            adjusted_lab[2] *= saturation_factor;
        }
    }
    
    // Single LAB->RGB conversion
    let new_rgb = lab_to_rgb(adjusted_lab);
    Rgb([
        new_rgb[0],
        new_rgb[1], 
        new_rgb[2],
    ])
}