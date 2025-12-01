// Uses template of commit <a54ffa4e351284ae031c1a2d52e681b386cb136e>

mod file_reading;
use file_reading::load_file;

mod part1;
mod part2;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match load_file(&args) {
        Ok(input) => {
            if args[1] == "1" {
                match part1::solution(&input) {
                    Ok(solution) => {
                        println!("\nPart 1 solution: {}", solution);
                    }
                    Err(e) => {
                        eprintln!("\nPart 1 error: {}", e);
                        std::process::exit(1);
                    }
                }
            } else if args[1] == "2" {
                match part2::solution(&input) {
                    Ok(solution) => {
                        println!("\nPart 2 solution: {}", solution);
                    }
                    Err(e) => {
                        eprintln!("\nPart 2 error: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("Invalid argument. Please use 1 or 2.");
                std::process::exit(1);
            }
        }
        Err(message) => {
            eprintln!("Error: {}", message);
            std::process::exit(1);
        }
    }
}
