use image::{GenericImage, GenericImageView, Rgba};
use opencv::{
    core::Mat,
    hub_prelude::MatTraitConst,
    imgcodecs::{imread, imwrite, IMREAD_COLOR},
    imgproc::{self, INTER_LINEAR},
};

pub fn opencv_merge(
    input1: &str,
    input2: &str,
    output_image: &str,
    merge_factor: f64,
) -> opencv::Result<()> {
    assert!(merge_factor <= 1.0);
    assert!(merge_factor >= 0.0);

    let image1: Mat = imread(input1, IMREAD_COLOR)?;
    let image2: Mat = imread(input2, IMREAD_COLOR)?;
    image1.cols();
    image2.size().unwrap();
    let mut image2_sized: Mat = Mat::default();
    let mut processed = Mat::default();
    imgproc::resize(
        &image2,
        &mut image2_sized,
        image1.size().unwrap(),
        0.0,
        0.0,
        INTER_LINEAR,
    )?;

    opencv::core::add_weighted(
        &image1,
        merge_factor,
        &image2_sized,
        1.0 - merge_factor,
        0.0,
        &mut processed,
        -1,
    )?;

    let params = opencv::core::Vector::new();
    imwrite(output_image, &processed, &params)?;
    Ok(())
}

/// modified GPT implementation https://chat.openai.com/share/645c86ad-93ca-405c-9b0f-1dddde67eb84
pub fn manual_merge(
    input1: &str,
    input2: &str,
    output_image: &str,
    merge_factor: f64,
) -> Result<(), image::ImageError> {
    // Load the input images
    let img1 = image::open(input1)?;
    let img2 = image::open(input2)?;

    // Ensure that both images have the same dimensions
    assert!(img1.dimensions() == img2.dimensions());

    // Create a new image to store the blended result
    let mut blended_image = image::DynamicImage::new_rgba8(img1.width(), img1.height());

    // Iterate over each pixel and perform the blending
    for y in 0..img1.height() {
        for x in 0..img1.width() {
            let pixel1 = img1.get_pixel(x, y).0;
            let pixel2 = img2.get_pixel(x, y).0;

            // Blend the RGB components
            let blended_rgb = (
                ((1.0 - merge_factor) * f64::from(pixel1[0]) + merge_factor * f64::from(pixel2[0]))
                    as u8,
                ((1.0 - merge_factor) * f64::from(pixel1[1]) + merge_factor * f64::from(pixel2[1]))
                    as u8,
                ((1.0 - merge_factor) * f64::from(pixel1[2]) + merge_factor * f64::from(pixel2[2]))
                    as u8,
            );

            // Create a new blended pixel
            let blended_pixel = Rgba([
                blended_rgb.0,
                blended_rgb.1,
                blended_rgb.2,
                pixel1[3], // Use alpha from the first image
            ]);

            // Set the pixel in the blended image
            blended_image.put_pixel(x, y, blended_pixel);
        }
    }

    // Save the blended image to the specified output path
    blended_image.save(output_image)?;

    Ok(())
}
