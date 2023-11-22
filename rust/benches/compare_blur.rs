use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use image_processing::blur::{opencv_box_blur, opencv_gaussian_blur};

pub fn compare_blur(c: &mut Criterion) {
    let input = "../test1.jpg";
    let output = "../test1_bench_blur.jpg";
    let blur_factor: i32 = 20;
    let mut group = c.benchmark_group("blur");
    group.bench_with_input(
        BenchmarkId::new("opencv_box_blur", 1),
        &(input, output, blur_factor),
        |b, (input, output, blur_factor)| {
            b.iter(|| opencv_box_blur(input, output, blur_factor.to_owned()).unwrap())
        },
    );
    group.bench_with_input(
        BenchmarkId::new("opencv_gaussian_blur", 1),
        &(input, output, blur_factor),
        |b, (input, output, blur_factor)| {
            b.iter(|| opencv_gaussian_blur(input, output, blur_factor.to_owned()).unwrap())
        },
    );
}

criterion_group!(benches, compare_blur);
criterion_main!(benches);
