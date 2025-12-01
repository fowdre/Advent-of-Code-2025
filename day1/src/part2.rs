pub fn solution(input: &[String]) -> Result<i128, &str> {
    let mut result: i128 = 0;
    let mut current_dial_pos: i128 = 50;

    for line in input {
        println!("\n{}", line);

        if line.chars().nth(0).unwrap() == 'L' {
            // negative
            let mut local_line = line.clone();
            local_line.remove(0);
            let parsed = local_line.parse::<i128>().unwrap();
            current_dial_pos -= parsed;

            loop {
                dbg!(current_dial_pos);
                if (0..=99).contains(&current_dial_pos) {
                    break;
                } else {
                    current_dial_pos += 100;
                    result += 1;
                }
            }
        } else {
            // positive
            let mut local_line = line.clone();
            local_line.remove(0);
            let parsed = local_line.parse::<i128>().unwrap();
            current_dial_pos += parsed;

            loop {
                dbg!(current_dial_pos);
                if (0..=99).contains(&current_dial_pos) {
                    break;
                } else {
                    current_dial_pos -= 100;
                    result += 1;
                }
            }
        }
    }

    Ok(result)
}
