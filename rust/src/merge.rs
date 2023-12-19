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
