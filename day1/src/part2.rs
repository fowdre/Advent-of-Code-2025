pub fn solution(input: &[String]) -> Result<i128, &str> {
    let mut result: i128 = 0;
    let mut current_dial_pos: i128 = 50;

    for line in input {
        println!("\n{}", line);

        if line.chars().nth(0).unwrap() == 'L' {
            // negative
            let mut local_line = line.clone();
            local_line.remove(0);
            let mut parsed = local_line.parse::<i128>().unwrap().abs();

            dbg!(current_dial_pos);

            while parsed > 0 {
                parsed -= 1;
                current_dial_pos -= 1;

                if current_dial_pos == 0 {
                    result += 1;
                }

                // wrap
                if current_dial_pos == -1 {
                    current_dial_pos = 99;
                }
            }

            dbg!(current_dial_pos);
        } else {
            // positive
            let mut local_line = line.clone();
            local_line.remove(0);
            let mut parsed = local_line.parse::<i128>().unwrap().abs();

            dbg!(current_dial_pos);

            while parsed > 0 {
                parsed -= 1;
                current_dial_pos += 1;

                // wrap
                if current_dial_pos == 100 {
                    current_dial_pos = 0;
                    result += 1;
                }
            }

            dbg!(current_dial_pos);
        }
    }

    Ok(result)
}
