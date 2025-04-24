use std::fs;
use regex::Regex;

fn main() {

    let input = fs::read_to_string("input.txt").expect("no file found");

    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();

    let mut sum = 0;

    let mut should_mul = true;

    for m in re.find_iter(&input) {

        let text = m.as_str();

        if text == "don't()" {
            should_mul = false
        } else if text == "do()" {
            should_mul = true
        }

        if should_mul && text.starts_with("mul") {
            let mut mul = text.split(",");

            let mut lhs = mul.next().unwrap();
            lhs = &lhs[4..];
            let mut rhs = mul.next().unwrap();
            rhs = &rhs[..rhs.len()-1];
    
            let lhs = lhs.parse::<i32>().unwrap();
            let rhs = rhs.parse::<i32>().unwrap();
    
            sum += lhs * rhs;
        }




    }

    println!("{}", sum);

}
