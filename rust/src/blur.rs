use image;

use lodepng::{self, Error};
use opencv::{core, core::Mat, imgproc};
use rgb::RGBA8;
use std::cmp::{max, min};

// OpenCV implementations

fn opencv_read_and_write(
    input_file: &str,
    output_file: &str,
    processing: impl Fn(&Mat, &mut Mat) -> opencv::Result<()>,
) -> opencv::Result<()> {
    let input: opencv::prelude::Mat =
        opencv::imgcodecs::imread(input_file, opencv::imgcodecs::IMREAD_COLOR)?;

    let mut processed = Mat::default();

    processing(&input, &mut processed)?;

    let params = opencv::core::Vector::new();
    opencv::imgcodecs::imwrite(output_file, &processed, &params)?;

    Ok(())
}

pub fn opencv_box_blur(
    input_file: &str,
    output_file: &str,
    blur_factor: i32,
) -> opencv::Result<()> {
    let img = opencv::imgcodecs::imread(input_file, opencv::imgcodecs::IMREAD_COLOR)?;

    let mut blurred = Mat::default();

    imgproc::blur(
        &img,
        &mut blurred,
        core::Size::new(blur_factor, blur_factor),
        core::Point::new(-1, -1),
        core::BORDER_DEFAULT,
    )?;

    let params = opencv::core::Vector::new();
    opencv::imgcodecs::imwrite(output_file, &blurred, &params)?;

    Ok(())
}

pub fn opencv_gaussian_blur(
    input_file: &str,
    output_file: &str,
    blur_factor: i32,
) -> opencv::Result<()> {
    opencv_read_and_write(input_file, output_file, |input, processed: &mut Mat| {
        imgproc::gaussian_blur(
            &input,
            processed,
            core::Size_ {
                width: blur_factor,
                height: blur_factor,
            },
            0.0,
            0.0,
            core::BORDER_DEFAULT,
        )
    })
}

// `image` library implementations

pub fn imagelib_gaussian_blur(
    input_file: &str,
    output_file: &str,
    sigma: f32,
) -> image::ImageResult<()> {
    let img = image::open(input_file)?;
    let img = img.blur(sigma);
    img.save(output_file)?;
    Ok(())
}

// Manual implementations

// blur implemented previously by Danylo adapted to rust https://github.com/DanBrazhnyk/image-processing/blob/js-project/blur.js
pub fn box_blur(input_file: &str, output_file: &str, radius: i32) -> Result<(), Error> {
    let input = lodepng::decode32_file(input_file)?;

    let data = input.buffer;
    let width = input.width;
    let height = input.height;

    let box_width = radius / 2;
    let box_height = radius / 2;

    let mut blurred: Vec<RGBA8> = Vec::with_capacity(data.len());

    for y in 0..height {
        for x in 0..width {
            let x = x as i32;
            let y = y as i32;

            let mut red: usize = 0;
            let mut green: usize = 0;
            let mut blue: usize = 0;
            // let mut alpha: u8 = 0;
            let mut count = 0;

            for dy in -box_height..=box_height {
                for dx in -box_width..=box_width {
                    let nx = min(max(x + dx, 0), (width - 1) as i32);
                    let ny = min(max(y + dy, 0), (height - 1) as i32);

                    let offset = (ny * width as i32 + nx) as usize;
                    let RGBA8 { r, g, b, a } = data[offset];
                    red += r as usize;
                    green += g as usize;
                    blue += b as usize;
                    // alpha += a;
                    count += 1;
                }
            }
            let offset: usize = (y * width as i32 + x) as usize;

            blurred.push(RGBA8 {
                r: (red / count) as u8,
                g: (green / count) as u8,
                b: (blue / count) as u8,
                a: data[offset].a,
            })
        }
    }
    lodepng::encode32_file(output_file, &blurred, width, height)?;
    Ok(())
}
