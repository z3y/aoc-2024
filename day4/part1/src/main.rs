use std::fs;
use regex::Regex;

fn main() {

    let input = fs::read_to_string("input.txt").expect("no file found");

    let re1 = Regex::new(r"XMAS").unwrap();
    let re2 = Regex::new(r"SAMX").unwrap();

    let mut grid = vec![];

    for line in input.lines() {

        let mut row = vec![];
        line.chars().for_each(|x| row.push(x));
        grid.push(row);

    }

    let mut count = re1.find_iter(&input).count();
    count += re2.find_iter(&input).count();
    
    let mut transformed_grid = vec![];

    let row_length: usize = grid[0].len();

    for x in 0..row_length {

        let mut column = vec![]; 

        for y in 0..grid.len() {

            column.push(grid[y][x]);
        }

        transformed_grid.push(column);
    }


    for row in &transformed_grid {

        let row = row_to_string(&row);
        count += re1.find_iter(&row).count();
        count += re2.find_iter(&row).count();
    }


    //let mut transformed_grid_diagonal = vec![];

    let mut col_length = grid.len();

    let mut diagonals = vec![];
    
    for x in 0..row_length {
        
        let mut index = 0;
        let mut diagonal = vec![];
        for y in x..col_length {
            if index > row_length || y > col_length {
                break;
            }

            diagonal.push(grid[y][index]);
            index += 1;
        }
        // println!("{:#?}", diagonal);

        diagonals.push(diagonal);
    }

    diagonals.remove(0);


    for x in 0..row_length {
        
        let mut index = 0;
        let mut diagonal = vec![];
        for y in x..col_length {
            if index > row_length || y > col_length {
                break;
            }

            diagonal.push(grid[index][y]);
            index += 1;
        }

        // println!("{:#?}", diagonal);
        diagonals.push(diagonal);
    }

    let count_start = diagonals.len();
    
    for x in 0..row_length {
        
        let mut index = row_length-1;
        let mut diagonal = vec![];
        for y in x..col_length {
            if index > row_length || y > col_length {
                break;
            }

            diagonal.push(grid[y][index]);
            if index == 0 {
                break;
            }
            index -= 1;
            
        }

        // println!("{:#?}", diagonal);
        diagonals.push(diagonal);
    }

    diagonals.remove(count_start);

    for x in 0..row_length {
        
        let mut index = 0;
        let mut diagonal = vec![];
        for y in x..col_length {
            if index > row_length || y > col_length {
                break;
            }

            diagonal.push(grid[index][col_length-1 - y]);
            index += 1;
        }

        diagonals.push(diagonal);
    }
    

    for diagonal in diagonals {
        let diagonal = row_to_string(&diagonal);
        // println!("{:#?}", diagonal);
        count += re1.find_iter(&diagonal).count();
        count += re2.find_iter(&diagonal).count();
    }
    println!("{}", count);
}

fn row_to_string(row: &Vec::<char>) -> String {
    row.iter().collect::<String>()
}
