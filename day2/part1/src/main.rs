use std::fs;

fn main() {

    let input = fs::read_to_string("input.txt").expect("no file found");

    let lines = input.lines();

    let mut safe_count = 0;
    for line in lines {

        let mut levels = line.split_ascii_whitespace();

        let mut previous_level = levels.next().unwrap().parse::<i32>().unwrap();
        
        let mut is_safe = true;
        let mut increasing = false;
        let mut decreasing = false;

        for level in levels {

            let level = level.parse::<i32>().unwrap();
            
            if level == previous_level {
                is_safe = false;
                break;
            }
            
            if level > previous_level {
                increasing = true;
            } else {
                decreasing = true;
            }

            let difference = (level - previous_level).abs();

            if difference > 3 {
                is_safe = false;
                break;
            }

            if decreasing && increasing {
                is_safe = false;
                break;
            }

            previous_level = level;
        }

        if is_safe {
            safe_count += 1;
        }
    }

    println!("{}", safe_count);
}
