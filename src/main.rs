use std::env;
use rust_course_hw01_lib::area;
use rust_course_hw01_lib::volume;
use rust_course_hw01_lib::area::TwoDObject;
use rust_course_hw01_lib::area::TwoDObject::{Circle, Rectangle};
use rust_course_hw01_lib::volume::ThreeDObject::{Sphere, Parallelepiped};

fn help(program_name: &String) {
    println!("The program figures out the Area for Circle and Rectangle or the Volume for Parallelepiped and Sphere");
    println!("Usage:");
    println!("{} Circle radius", program_name);
    println!("{} Rectangle width height", program_name);
    println!("{} Sphere radius", program_name);
    println!("{} Parallelepiped width height length", program_name);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = &args[0];
    if args.len() < 2 {
        help(program_name);
        return;
    }
    let figure = &args[1];
    let num1 = &args[2];
    let param1: f32 = match num1.parse() {
        Ok(n) => {
            n
        }
        Err(_) => {
            eprintln!("error: second param should be a number");
            help(program_name);
            return;
        }
    };

    if figure == "Circle" {
        let result = area::figure_out_area(Circle { radius: param1 });
        println!("Circle area is {}", result);
        return;
    }
    if figure == "Sphere" {
        let result = volume::figure_out_volume(Sphere { radius: param1 });
        println!("Sphere volume is {}", result);
        return;
    }
    let num2 = &args[3];
    let param2: f32 = match num2.parse() {
        Ok(n) => {
            n
        }
        Err(_) => {
            eprintln!("error: third param should be a number");
            help(program_name);
            return;
        }
    };
    if figure == "Rectangle" {
        let result = area::figure_out_area(Rectangle { width: param1, height: param2 });
        println!("Rectangle area is {}", result);
        return;
    }
    let num3 = &args[4];
    let param3: f32 = match num3.parse() {
        Ok(n) => {
            n
        }
        Err(_) => {
            eprintln!("error: fourth param should be a number");
            help(program_name);
            return;
        }
    };
    if figure == "Parallelepiped" {
        let result = volume::figure_out_volume(Parallelepiped { width: param1, height: param2, length: param3 });
        println!("Parallelepiped volume is {}", result);
        return;
    }
    help(program_name);
}
