use std::fs;
use regex::Regex;

fn main() {

    let input = fs::read_to_string("input.txt").expect("no file found");

    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    let mut sum = 0;
    for m in re.find_iter(&input) {

        let mut text = m.as_str().split(",");

        let mut lhs = text.next().unwrap();
        lhs = &lhs[4..];
        let mut rhs = text.next().unwrap();
        rhs = &rhs[..rhs.len()-1];

        let lhs = lhs.parse::<i32>().unwrap();
        let rhs = rhs.parse::<i32>().unwrap();

        sum += lhs * rhs;


    }

    println!("{}", sum);

}
