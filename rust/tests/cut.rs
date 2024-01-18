#[cfg(test)]

mod tests {
    use image_processing::cut;
    use opencv::core::{Point, Scalar};

    #[test]
    fn cut() {
        let input = "../test1.png";
        fn output(name: &str) -> String {
            format!("../test1_{}_cut.png", name)
        }

        let coordinates = vec![
            Point::new(10, 10),
            Point::new(100, 100),
            Point::new(50, 100),
        ];

        let white = Scalar::new(255.0, 255.0, 255.0, 255.0);
        let black_rgba = [255, 255, 255, 255];

        assert!(cut::opencv_cut(input, &output("opencv"), white, coordinates).is_ok());
        assert!(cut::manual_cut(input, &output("manual"), 20, 30, 400, black_rgba).is_ok());
    }
}