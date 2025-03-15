use std::fs;

fn main() {

    let input = fs::read_to_string("input.txt").expect("no file found");

    let lines = input.lines();

    let mut left = vec![];
    let mut right = vec![];

    for line in lines {
        let mut split = line.split_whitespace();
        

        let left_string = split.next().unwrap();
        let right_string = split.next().unwrap();
        
        left.push(left_string.parse::<i32>().unwrap());
        right.push(right_string.parse::<i32>().unwrap());

    }

    left.sort();
    right.sort();

    let mut total_distance = 0;

    for (l, r) in left.iter().zip(&right) {
        total_distance += (l - r).abs();
    }

    
    println!("Total Distance: {}", total_distance);
}
