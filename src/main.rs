use std::env;
use std::process;

use edit_distance::min_edit_distance;
use edit_distance::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let med = min_edit_distance(&config).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1);
    });

    println!(
        "The minimium editing distance for trasforming s1={} to s2={} is {}",
        config.s1, config.s2, med
    );
}
