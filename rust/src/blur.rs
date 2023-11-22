use opencv::{core, core::Mat, imgproc};

fn opencv_read_and_write(
    input_file: &str,
    output_file: &str,
    processing: impl Fn(&Mat, &mut Mat) -> opencv::Result<()>,
) -> opencv::Result<()> {
    let input: opencv::prelude::Mat =
        opencv::imgcodecs::imread(input_file, opencv::imgcodecs::IMREAD_COLOR)?;

    let mut processed = Mat::default();

    processing(&input, &mut processed).expect("applying effect failed");

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
