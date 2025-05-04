use std::{cmp::Ordering, fs};
use regex::Regex;

struct Rule {
    l: i32,
    r: i32
}

fn main() {

    let input = fs::read_to_string("input.txt").expect("no file found");

    let mut rules = vec![];

    let mut parsing_rules = true;

    let mut sum = 0;
    for line in input.lines() {

        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        if parsing_rules {
            let mut split = line.split("|");

            let l = split.next().unwrap().parse::<i32>().unwrap();
            let r = split.next().unwrap().parse::<i32>().unwrap();

            let rule = Rule{l,r};

            rules.push(rule);
        }
        else {
            let mut pages = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

            let mut rule_broken = false;
            for rule in rules.iter() {
                
                let mut l_exists = false;
                let mut r_exists = false;

                for page in pages.clone() {
                    if page == rule.l {
                        l_exists = true;
                    }
                    if page == rule.r {
                        r_exists = true;
                    }
                }

                if l_exists && r_exists {

                    let l_index = pages.iter().position(|x| *x == rule.l).unwrap();
                    let r_index = pages.iter().position(|x| *x == rule.r).unwrap();

                    if l_index > r_index {
                        rule_broken = true;
                        break;
                    }
                }
            }

            pages.sort_by(|a, b| compare(*a, *b, &rules));

            if rule_broken {
                let middle_index = pages.len() / 2;

                println!("{:#?}", pages);


                sum += pages[middle_index]
            }
        }
    }

    println!("{}", sum);
 
}

fn compare(a: i32, b:i32, rules: &Vec<Rule>) -> Ordering {

    for rule in rules {
        
        if rule.l == a && rule.r == b {
            return Ordering::Less;
        }
        if rule.r == a && rule.l == b {
            return Ordering::Greater;
        }
    }

    Ordering::Equal
}