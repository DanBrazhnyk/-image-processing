use image_processing::{blur, cut, merge};

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
    let merge_factor: f64 = args[3].parse().expect("merge factor needs to be a float!");

    match merge::manual_merge(image1, image2, output, merge_factor) {
        Ok(_) => (),
        Err(e) => println!("ERROR: {}", e),
    }
}

fn handle_cut(args: &[String]) {
    if args.is_empty() {
        println!("cut [input] [output] [x] [y] [side_length] [color rgba separated with ,]");
        exit(0)
    }
    let [input, output, x, y, side_length, color_str] = args else {
        println!("improper arguments");
        exit(1)
    };
    let x: usize = x.parse().unwrap();
    let y: usize = y.parse().unwrap();
    let side_length: usize = side_length.parse().unwrap();
    let color: [u8; 4] = color_str
        .split(",")
        .map(|a| a.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap();
    match cut::manual_cut(&input, &output, x, y, side_length, color) {
        Ok(_) => (),
        Err(e) => println!("ERROR: {}", e),
    }
}

fn handle_blur(args: &[String]) {
    if args.is_empty() {
        println!("merge [input1] [input2] [output] [merge factor]");
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
