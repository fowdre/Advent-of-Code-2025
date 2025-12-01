pub fn solution(input: &[String]) -> Result<i128, &str> {
    let mut result: i128 = 0;
    let mut current_dial_pos: i128 = 50;

    for line in input {
        if line.chars().nth(0).unwrap() == 'L' {
            // negative
            let mut local_line = line.clone();
            local_line.remove(0);
            let mut parsed = local_line.parse::<i128>().unwrap().abs();

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
        } else {
            // positive
            let mut local_line = line.clone();
            local_line.remove(0);
            let mut parsed = local_line.parse::<i128>().unwrap().abs();

            while parsed > 0 {
                parsed -= 1;
                current_dial_pos += 1;

                // wrap
                if current_dial_pos == 100 {
                    current_dial_pos = 0;
                    result += 1;
                }
            }
        }
    }

    Ok(result)
}
