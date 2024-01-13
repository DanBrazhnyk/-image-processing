use opencv::{
    core::Point,
    imgproc::{fill_convex_poly, LINE_8},
};

pub fn cut(
    input_file: &str,
    output_file: &str,
    color: opencv::core::Scalar,
    coordinates: Vec<Point>,
) -> std::result::Result<(), opencv::Error> {
    let mut input: opencv::prelude::Mat =
        opencv::imgcodecs::imread(input_file, opencv::imgcodecs::IMREAD_COLOR)?;

    let coords_vec: opencv::core::Vector<Point> = opencv::core::Vector::from(coordinates);
    fill_convex_poly(&mut input, &coords_vec, color, LINE_8, 0)?;

    let params = opencv::core::Vector::new();
    opencv::imgcodecs::imwrite(output_file, &input, &params)?;

    Ok(())
}
