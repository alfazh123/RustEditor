use wasm_bindgen::prelude::*;
use std::{io::Cursor};
use image::{GenericImageView, ImageBuffer, Rgb};
use colors_transform::{Rgb as ColorTransformRgb, Hsl, Color}; // Import trait ColorTransform

#[wasm_bindgen]
pub fn resize(image_data: &[u8], height: u32, width: u32) -> Vec<u8> {
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    let resized_image = image.resize(width, height, image::imageops::FilterType::Lanczos3);
    let mut buf = Cursor::new(Vec::new());
    resized_image.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write the image");
    buf.into_inner()
}

#[wasm_bindgen]
pub fn resize_exact(image_data: &[u8], width: u32, height: u32) -> Vec<u8> {
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    let resized_image = image.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
    let mut buf = Cursor::new(Vec::new());
    resized_image.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write the image");
    buf.into_inner()
}

#[wasm_bindgen]
pub fn resize_one_side(image_data: &[u8], width: u32, height: u32) -> Vec<u8> {
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    let resized_image = image.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
    let mut buf = Cursor::new(Vec::new());
    resized_image.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write the image");
    return buf.into_inner();
}

#[wasm_bindgen]
pub fn crop(image_data: &[u8], x: u32, y: u32, width: u32, height: u32) -> Vec<u8> {
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    let cropped_image = image.crop_imm(x, y, width, height);
    let mut buf = Cursor::new(Vec::new());
    cropped_image.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write the image");
    buf.into_inner()
}

#[wasm_bindgen]
pub fn change_scale_image(image_data: &[u8], scale: f32) -> Vec<u8> {
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    let width = (image.width() as f32 * scale) as u32;
    let height = (image.height() as f32 * scale) as u32;
    let resized_image = image.resize(width, height, image::imageops::FilterType::Lanczos3);
    let mut buf = Cursor::new(Vec::new());
    resized_image.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write the image");
    buf.into_inner()
}

#[wasm_bindgen]
pub fn get_size(image_data: &[u8]) -> Vec<u32> {
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    // let image = image::load_from_memory(fix_size_image(image_data).as_slice()).expect("Failed to open the file");
    let (width, height) = image.dimensions();
    return vec![width, height];
}

#[wasm_bindgen]
pub fn switch_color(image_source: &[u8], image_reference: &[u8]) -> Vec<u8> {
    // let source_image = ImageReader::open("src/star1.jpg")?.decode()?;
    let source_image = image::load_from_memory(image_source).expect("Failed to open the file");
    let _source_gray = source_image.to_luma8();
    let source_rgb = source_image.to_rgb8();
    // let reference_image = ImageReader::open("src/blade-runner2.png")?.decode()?;
    let reference_image = image::load_from_memory(image_reference).expect("Failed to open the file");
    let _reference_gray = reference_image.to_luma8();
    let reference_rgb = reference_image.to_rgb8();
    let (width, height) = source_rgb.dimensions();

    let source_lab = image_rgb_to_lab(&source_rgb);
    let reference_lab = image_rgb_to_lab(&reference_rgb);

    let source_stats = image_stats(&source_lab);
    let reference_stats = image_stats(&reference_lab);

    let mut new_image: image::RgbImage = ImageBuffer::new(width, height);

    let mut id = 0;

    for y in 0..height {
        for x in 0..width {
            let lab = source_lab[id];
            let mut new_lab = [0.0; 3];
            for i in 0..3 {
                let original = lab[i];
                let transform = (lab[i] - source_stats.mean[i]) / source_stats.stddev[i].max(1e-3) * reference_stats.stddev[i] + reference_stats.mean[i];
                new_lab[i] = original * (1.0 - 0.5) + transform * 0.5;
                // new_lab[i] = (transform + original) * 0.5;
                // new_lab[i] = (original + transform) / 3.0;
                // println!("{}: {} - {} / {} * {} + {}, {}", id, lab[i], source_stats.mean[i], source_stats.stddev[i], reference_stats.stddev[i], reference_stats.mean[i], new_lab[i]);
                // println!("{}", new_lab[i]);
            }
            let new_rgb = lab_to_rgb(new_lab);
            new_image.put_pixel(x, y, Rgb([new_rgb[0], new_rgb[1], new_rgb[2]]));
            println!("{}, {}: {} {} {}", x, y, new_rgb[0], new_rgb[1], new_rgb[2]);
            id += 1;
        }
    }

    // new_image.save("src/matched.png")?;
    let mut buf = Cursor::new(Vec::new());
    new_image.write_to(&mut buf, image::ImageFormat::Png).expect("Failed to write the image");
    buf.into_inner()
}

struct ImageStats {
    mean: [f64; 3],
    stddev: [f64; 3],
}

fn image_stats(image: &Vec<[f64; 3]>) -> ImageStats {
    let mut mean = [0.0; 3];
    let mut stddev = [0.0; 3];

    for pixel in image.iter() {
        for i in 0..3 {
            mean[i] += pixel[i];
        }
    }

    for i in 0..3 {
        mean[i] /= image.len() as f64;
    }

    for pixel in image.iter() {
        for i in 0..3 {
            stddev[i] += (pixel[i] - mean[i]).powi(2);
        }
    }

    for i in 0..3 {
        stddev[i] = (stddev[i] / image.len() as f64).sqrt().max(1e-3);
    }

    ImageStats {
        mean,
        stddev,
    }
}

fn lab_to_rgb(lab: [f64; 3]) -> [u8; 3] {
    let fy = (lab[0] + 16.0) / 116.0;
    let fx = (lab[1] / 500.0) + fy;
    let fz = fy - (lab[2] / 200.0);

    let x = if fx.powf(3.0) > 0.008856 {
        fx.powf(3.0)
    } else {
        (116.0 * fx - 16.0) / 903.3
    };

    let y = if lab[0] > (0.008865 * 903.3) {
        ((lab[0] + 16.0) / 116.0).powf(3.0)
    } else {
        lab[0] / 903.3
    };

    let z = if fz.powf(3.0) > 0.008856 {
        fz.powf(3.0)
    } else {
        (116.0 * fz - 16.0) / 903.3
    };

    let x = x * 0.95047;
    let y = y * 1.00000;
    let z = z * 1.08883;

    // 3.2404542 -1.5371385 -0.4985314
    // -0.9692660  1.8760108  0.0415560
    // 0.0556434 -0.2040259  1.0572252

    let r = x * 3.2404542 + y * -1.5371385 + z * -0.4985314;
    let g = x * -0.9692660 + y * 1.8760108 + z * 0.0415560;
    let b = x * 0.0556434 + y * -0.2040259 + z * 1.0572252;

    let r = if r <= 0.0031308 {
        12.92 * r
    } else {
        1.055 * r.powf(1.0 / 2.4) - 0.055
    };

    let g = if g <= 0.0031308 {
        12.92 * g
    } else {
        1.055 * g.powf(1.0 / 2.4) - 0.055
    };

    let b = if b <= 0.0031308 {
        12.92 * b
    } else {
        1.055 * b.powf(1.0 / 2.4) - 0.055
    };

    [
        (r * 255.0).clamp(0.0, 255.0) as u8,
        (g * 255.0).clamp(0.0, 255.0) as u8,
        (b * 255.0).clamp(0.0, 255.0) as u8
    ]

}

fn image_rgb_to_lab(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<[f64; 3]> {
    let mut lab_image = Vec::with_capacity(image.pixels().count());

    for pixel in image.pixels() {
        let lab = rgb_to_lab(*pixel);

        lab_image.push(lab);
    }

    lab_image
}

fn rgb_to_lab(pixel: Rgb<u8>) -> [f64; 3] {
    let r = pixel[0] as f64 /255.0;
    let g = pixel[1] as f64 /255.0;
    let b = pixel[2] as f64 /255.0;
    
    let r_linear = if r > 0.040450 {
        ((r + 0.055) / 1.055).powf(2.4)
    } else {
        r / 12.92
    };
    
    let g_linear = if g > 0.040450 {
        ((g + 0.055) / 1.055).powf(2.4)
    } else {
        g / 12.92
    };
    
    let b_linear = if b > 0.040450 {
        ((b + 0.055) / 1.055).powf(2.4)
    } else {
        b / 12.92
    };
    
    let x = r_linear * 0.4124564 + g_linear * 0.3575761 + b_linear * 0.1804375;
    let y = r_linear * 0.2126729 + g_linear * 0.7151522 + b_linear * 0.0721750;
    let z = r_linear * 0.0193339 + g_linear * 0.1191920 + b_linear * 0.9503041;
    
    // Reference white point D65 (0.95046, 1.0, 1.08906).
    let x = x / 0.95047;
    let y = y / 1.00000;
    let z = z / 1.08883;
    
    // 0.576669  0.185558  0.188229
    // 0.297345  0.627364  0.075291
    // 0.027031  0.070689  0.991338
    
    let fx = if x > 0.008856 {
        (x).powf(1.0 / 3.0)
    } else {
        x * 7.787 + 16.0 / 116.0
    };
    let fy = if y > 0.008856 {
        (y).powf(1.0 / 3.0)
    } else {
        y * 7.787 + 16.0 / 116.0
    };
    
    let fz = if z > 0.008856 {
        (z).powf(1.0 / 3.0)
    } else {
        z * 7.787 + 16.0 / 116.0
    };
    
    let l = 116.0 * fy - 16.0;
    let a = 500.0 * (fx - fy);
    let b = 200.0 * (fy - fz);
    
    let lab = [
        l,
        a,
        b,
    ];
    lab
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

#[wasm_bindgen]
pub fn adjust_exposure_image(image_data: &[u8], exposure: f64) -> Vec<u8> {
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

#[wasm_bindgen]
pub fn adjust_contrasts_image(image_data: &[u8], contrasts: f32) -> Vec<u8> {
    let image = image::load_from_memory(image_data).expect("Failed to open the file");
    let rgb_image = image.to_rgb8();
    let (width, height) = rgb_image.dimensions();
    let mut adjusted_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    let contrasts_value: f32;

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

fn adjust_contrast(pixel: Rgb<u8>, contrast_factor: f32) -> Rgb<u8> {
    let r = pixel[0] as f32;
    let g = pixel[1] as f32;
    let b = pixel[2] as f32;
    
    // Apply contrast adjustment: new_value = (old_value - 128) * factor + 128
    // 128 is the middle gray point (0.5 * 255)
    let adjusted_r = ((r - 128.0) * contrast_factor + 128.0).clamp(0.0, 255.0);
    let adjusted_g = ((g - 128.0) * contrast_factor + 128.0).clamp(0.0, 255.0);
    let adjusted_b = ((b - 128.0) * contrast_factor + 128.0).clamp(0.0, 255.0);
    Rgb([
        adjusted_r as u8,
        adjusted_g as u8,
        adjusted_b as u8,
    ])
}

#[wasm_bindgen]
pub fn adjust_temperature_image(image_data: &[u8], temperature: f64) -> Vec<u8> {
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

#[wasm_bindgen]
pub fn adjust_tint_image(image_data: &[u8], tint: f64) -> Vec<u8> {
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