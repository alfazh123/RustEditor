use std::{io::Cursor};
use image::{ImageBuffer, Rgb};
use crate::lab_converter::{lab_to_rgb, rgb_to_lab};

pub fn switch_color_handler(image_source: &[u8], image_reference: &[u8]) -> Vec<u8> {
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

fn image_rgb_to_lab(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<[f64; 3]> {
    let mut lab_image = Vec::with_capacity(image.pixels().count());

    for pixel in image.pixels() {
        let lab = rgb_to_lab(*pixel);

        lab_image.push(lab);
    }

    lab_image
}