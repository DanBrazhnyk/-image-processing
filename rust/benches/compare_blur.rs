use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use image_processing::blur;
use image_processing::blur::{imagelib_gaussian_blur, opencv_box_blur, opencv_gaussian_blur};

pub fn compare_blur(c: &mut Criterion) {
    let input = "../test1.png";
    let output = "../test1_bench_blur.png";
    let blur_start = 20;
    let blur_finish: i32 = 21;
    let mut group = c.benchmark_group("blur");

    for blr in (blur_start..blur_finish).step_by(2) {
        group.bench_with_input(
            BenchmarkId::new("opencv_box_blur", blr),
            &(input, output, blr),
            |b, (input, output, blr)| {
                b.iter(|| opencv_box_blur(input, output, blr.to_owned()).unwrap())
            },
        );
        group.bench_with_input(
            BenchmarkId::new("opencv_gaussian_blur", blr),
            &(input, output, blr),
            |b, (input, output, blr)| {
                b.iter(|| opencv_gaussian_blur(input, output, blr.to_owned()).unwrap())
            },
        );
        group.bench_with_input(
            BenchmarkId::new("imagelib_gaussian_blur", blr),
            &(input, output, blr),
            |b, (input, output, blr)| {
                b.iter(|| imagelib_gaussian_blur(input, output, blr.to_owned() as f32).unwrap())
            },
        );
        group.bench_with_input(
            BenchmarkId::new("manual_box_blur", blr),
            &(input, output, blr),
            |b, (input, output, blr)| {
                b.iter(|| blur::box_blur(input, output, blr.to_owned()).unwrap())
            },
        );
    }
}

criterion_group!(benches, compare_blur);
criterion_main!(benches);
