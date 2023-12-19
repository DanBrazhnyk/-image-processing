#[cfg(test)]
mod tests {
    use image_processing::merge;

    #[test]
    fn merge() {
        let input1 = "../test1.png";
        let input2 = "../test2.png";

        fn output(name: &str) -> String {
            format!("../test_{}_merge.png", name)
        }

        let merge_factor: f64 = 0.5;
        assert!(merge::opencv_merge(input1, input2, &output("opencv"), merge_factor).is_ok());
    }
}
