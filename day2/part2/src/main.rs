use std::fs;

fn main() {

    let input = fs::read_to_string("input.txt").expect("no file found");

    let lines = input.lines();

    let mut safe_count = 0;
    for line in lines {

        let levels_strings = line.split_ascii_whitespace();

        let mut levels = vec![];
        for ele in levels_strings {
            levels.push(ele.parse::<i32>().unwrap());
        }
        let len = levels.len();

        let mut is_safe = check_safe(&levels);
        if !is_safe {

            for removed_index in 0..len {
                
                let mut levels_copy = levels.clone();
                levels_copy.remove(removed_index);

                is_safe = check_safe(&levels_copy);

                if is_safe {
                    break;
                }
            }
        }

        println!("{} {}", line, is_safe);

        if is_safe {
            safe_count += 1;
        }
    }

    println!("{}", safe_count);
}

fn check_safe(levels: &Vec::<i32>) -> bool {

    let mut previous_level = levels.first().unwrap();

    let mut delta = 0;

    for level in levels[1..].iter() {

        let change = level - previous_level;
        let difference = change.abs();

        if difference > 3 {
            return false
        }

        delta += change.signum();

        previous_level = level;
    }

    delta.abs() == levels.len() as i32 - 1
}