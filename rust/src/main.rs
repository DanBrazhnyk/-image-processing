mod blur;
mod cut;
mod merge;

use blur::blur;
use cut::cut;
use merge::merge;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_help();
        exit(1);
    }

    // let a = ;

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
        println!("merge [image1] [image2] [image2 merge factor (float)]");
        exit(0)
    }

    if args.len() != 3 {
        panic!("wrong number of arguments!")
    }

    let merge_factor: f32 = args[2].parse().expect("merge factor needs to be a float!");

    merge(&args[0], &args[1], merge_factor);
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
    for arg in args {
        println!("{}", arg);
    }
}
