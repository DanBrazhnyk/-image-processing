#[cfg(test)]

mod tests {
    use image_processing::blur;

    #[test]
    fn blur() {
        let input = "../test1.jpg";
        fn output(name: &str) -> String {
            format!("../test1_{}_blur.jpg", name)
        }

        let sigma = 20.0;
        let blur_factor = 20;

        assert!(blur::imagelib_gaussian_blur(input, &output("imagelib_gaussian"), sigma).is_ok());
        assert!(blur::opencv_gaussian_blur(input, &output("opencv_gaussian"), blur_factor).is_ok());
        assert!(blur::opencv_box_blur(input, &output("opencv_box"), blur_factor).is_ok());
    }
}
