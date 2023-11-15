use opencv::{core, imgproc};

pub fn opencv_box_blur(
    input_file: &str,
    output_file: &str,
    blur_factor: i32,
) -> opencv::Result<()> {
    let img = opencv::imgcodecs::imread(input_file, opencv::imgcodecs::IMREAD_COLOR)?;

    let mut blurred = core::Mat::default();

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
}
