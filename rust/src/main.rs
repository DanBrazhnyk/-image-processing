use image_processing::blur;
use image_processing::merge;

use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_help();
        exit(1);
    }

    match args[1].as_ref() {
        "blur" => handle_blur(&args[2..]),
        "cut" => handle_cut(&args[2..]),
        "merge" => handle_merge(&args[2..]),
        _ => print_help(),
    }
}

fn print_help() {
    println!("<help goes here>");
}

fn handle_merge(args: &[String]) {
    if args.is_empty() {
        println!("merge [image1] [image2] [output image name] [image2 merge factor (float)]");
        exit(0)
    }

    if args.len() != 4 {
        panic!("wrong number of arguments!")
    }

    let image1 = &args[0];
    let image2 = &args[1];
    let output = &args[2];
    let merge_factor: f32 = args[3].parse().expect("merge factor needs to be a float!");

    merge::opencv_merge(input1, input2, output, merge_factor);
}

fn handle_cut(args: &[String]) {
    if args.is_empty() {
        println!("cut help");
        exit(0)
    }
    for arg in args {
        println!("{}", arg);
    }
}

fn handle_blur(args: &[String]) {
    if args.is_empty() {
        println!("merge help");
        exit(0)
    }

    let [input, output, blur_factor] = args else {
        println!("improper arguments");
        exit(1)
    };

    let blur_factor = match blur_factor.parse::<i32>() {
        Ok(val) => val,
        Err(_) => {
            println!("couldn't parse blur factor");
            exit(1)
        }
    };
    println!("bluring");

    match blur::box_blur(input, output, blur_factor) {
        Ok(_) => println!("done"),
        Err(e) => {
            println!("Blurring FAILED");
            println!("{}", e);
            exit(1)
        }
    };
}
