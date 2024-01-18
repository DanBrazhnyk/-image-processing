use image::GenericImageView;
use opencv::{
    core::Point,
    imgproc::{fill_poly, LINE_8},
};

pub fn opencv_cut(
    input_file: &str,
    output_file: &str,
    color: opencv::core::Scalar,
    coordinates: Vec<Point>,
) -> std::result::Result<(), opencv::Error> {
    let mut input: opencv::prelude::Mat =
        opencv::imgcodecs::imread(input_file, opencv::imgcodecs::IMREAD_COLOR)?;

    let coords_vec: opencv::core::Vector<Point> = opencv::core::Vector::from(coordinates);
    fill_poly(&mut input, &coords_vec, color, LINE_8, 0, Point::new(0, 0))?;

    let params = opencv::core::Vector::new();
    opencv::imgcodecs::imwrite(output_file, &input, &params)?;

    Ok(())
}

pub fn manual_cut(
    input_image: &str,
    output_image: &str,
    x: usize,
    y: usize,
    side_length: usize,
    color_rgba: [u8; 4],
) -> Result<(), image::ImageError> {
    // Load the input image
    let img = image::open(input_image)?;
    let (width, height) = img.dimensions();

    // Create a mutable RGBA image to modify
    let mut output_img =
        image::RgbaImage::from_raw(width, height, vec![0; width as usize * height as usize * 4])
            .expect("Failed to create output image");

    // Copy pixels from the input image to the output image
    for (ox, oy, pixel) in img.pixels() {
        output_img.put_pixel(ox, oy, pixel);
    }

    // Draw a square on the output image
    for sx in x..(x + side_length) {
        for sy in y..(y + side_length) {
            if sx < output_img.width() as usize && sy < output_img.height() as usize {
                output_img.put_pixel(sx as u32, sy as u32, image::Rgba(color_rgba));
            }
        }
    }

    // Save the modified image to the output file
    output_img.save(output_image)?;

    Ok(())
}
