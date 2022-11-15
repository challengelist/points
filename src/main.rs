use std::{
    env,
    process
};

use crate::calculator::PointCalculator;

mod calculator;
mod standard;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Store base variables.
    let mut position: i32 = 1;
    let mut victor_count: i32 = 1;
    let mut list_size: i32 = 1;

    if args.len() < 3 {
        println!("Usage: {} <position> <victor_count> <list_size>", args[0]);
        process::exit(1);
    }

    // Parse the position.
    match args[1].parse::<i32>() {
        Ok(num) => position = num,
        Err(_) => {
            println!("Error: Position must be a number.");
            process::exit(1);
        }
    }

    // Parse the victor count.
    match args[2].parse::<i32>() {
        Ok(num) => victor_count = num,
        Err(_) => {
            println!("Error: Victor count must be a number.");
            process::exit(1);
        }
    }

    // Parse the list size.
    match args[3].parse::<i32>() {
        Ok(num) => list_size = num,
        Err(_) => {
            println!("Error: List size must be a number.");
            process::exit(1);
        }
    }

    // Create the calculator.
    let calculator = standard::StandardPointCalculator {
        position: position,
        victor_count: victor_count,
        list_size: list_size
    };

    // Calculate the points.
    let points = calculator.calculate();

    // Print the points.
    println!("Points Worth: {}", points);

}
