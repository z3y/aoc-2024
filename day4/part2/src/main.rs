use std::fs;

fn main() {

    let input = fs::read_to_string("input.txt").expect("no file found");

    let mut grid = vec![];

    for line in input.lines() {

        let mut row = vec![];
        line.chars().for_each(|x| row.push(x));
        grid.push(row);
    }

    let mut count = 0;

    let mut len = grid.len();

    for x in 0..len {
        for y in 0..len {

            let current = grid[x][y];

            
            if current != 'A' {
                continue;
            }
            
            let x = x as i32;
            let y = y as i32;

            if !inside_bounds(x-1,y-1,len) {
                continue;
            }
            if !inside_bounds(x+1,y+1,len) {
                continue;
            }
            

            let x = x as usize;
            let y = y as usize;

            let m00 = grid[x-1][y-1];
            let m02 = grid[x+1][y-1];

            let m11 = grid[x][y];

            let m20 = grid[x-1][y+1];
            let m22 = grid[x+1][y+1];

            if format!("{}{}{}", m00, m11, m22) == "MAS" && format!("{}{}{}", m02, m11, m20) == "MAS" {
                count += 1;
                continue;
            }

            if format!("{}{}{}", m00, m11, m22) == "SAM" && format!("{}{}{}", m02, m11, m20) == "SAM" {
                count += 1;
                continue;
            }

            if format!("{}{}{}", m00, m11, m22) == "SAM" && format!("{}{}{}", m02, m11, m20) == "MAS" {
                count += 1;
                continue;
            }

            if format!("{}{}{}", m00, m11, m22) == "MAS" && format!("{}{}{}", m02, m11, m20) == "SAM" {
                count += 1;
                continue;
            }

        }
    }

    println!("{}", count);

}

fn inside_bounds(x: i32, y: i32, length: usize) -> bool {
    x >= 0 && y >= 0 && x < length as i32 && y < length as i32
}