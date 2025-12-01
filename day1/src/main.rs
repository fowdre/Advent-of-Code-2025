mod file_reading;
use file_reading::load_file;

mod part1;
mod part2;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match load_file(&args) {
        Ok(input) => {
            if args[1] == "1" {
                let part1_result = part1::solution(&input);

                println!("Part 1 solution: {}", part1_result);
            } else if args[1] == "2" {
                let part2_result = part2::solution(&input);

                println!("Part 2 solution: {}", part2_result);
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
