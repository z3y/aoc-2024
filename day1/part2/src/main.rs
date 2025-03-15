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

    let mut similarity_scores = vec![];

    for l in &left {

        let mut count = 0;
        for r in &right {
            if l == r  {
                count += 1;
            }
        }

        similarity_scores.push(l * count);
    }

    let total = similarity_scores.iter().sum::<i32>();
    
    println!("Total Distance: {}", total);
}
