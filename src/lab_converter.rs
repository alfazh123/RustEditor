use image::{Rgb};

pub fn lab_to_rgb(lab: [f64; 3]) -> [u8; 3] {
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

pub fn rgb_to_lab(pixel: Rgb<u8>) -> [f64; 3] {
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
