pub fn solution(input: &[String]) -> Result<u128, &str> {
    let mut result: u128 = 0;
    let mut current_dial_pos: i128 = 50;

    for line in input {
        if line.chars().nth(0).unwrap() == 'L' {
            // negative
            let mut local_line = line.clone();
            local_line.remove(0);
            let parsed = local_line.parse::<i128>().unwrap();

            current_dial_pos -= parsed;
            current_dial_pos = current_dial_pos.rem_euclid(100);

            if current_dial_pos == 0 {
                result += 1;
            }
        } else {
            // positive
            let mut local_line = line.clone();
            local_line.remove(0);
            let parsed = local_line.parse::<i128>().unwrap();

            current_dial_pos += parsed;
            current_dial_pos = current_dial_pos.rem_euclid(100);

            if current_dial_pos == 0 {
                result += 1;
            }
        }
    }

    Ok(result)
}
