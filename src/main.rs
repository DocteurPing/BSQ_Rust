use std::env;
use std::fs;

struct BestSquare{
    index: usize,
    size: usize,
}

fn algo(map: &mut String, line_size: usize) {
    let mut best_square = BestSquare{index: 0, size: 0};
    for i in 0..map.len() {
        if map.chars().nth(i).unwrap() == '.' {
            check_square_size(map, &mut best_square, line_size, i);
        }
    }
    put_cross(&mut best_square, map, line_size);
}

fn put_cross(best_square: &mut BestSquare, map: &mut String, line_size: usize) {
    for i in 0..best_square.size {
        for j in 0..best_square.size {
            let index = i * line_size + best_square.index + j;
            map.replace_range(index..index + 1, "x");
        }
    }
    println!("{}", map);
}

fn check_square_size(map: &str, best_square: &mut BestSquare, line_size: usize, index: usize) {
    println!("Hello, world! {}", index);
    let mut size = best_square.size + 1;
    loop {
        for i in 0..size {
            for j in 0..size {
                if j >= line_size {
                    return;
                }
                match map.chars().nth(i * line_size + j + index) {
                    None => { return; }
                    Some(char) => {
                        if char != '.' {
                            return;
                        }
                    }
                };

            }
        }
        best_square.size = size;
        best_square.index = index;
        size += 1;
    }
}

fn main() {
    if let Some(path) = env::args().nth(1) {
        let mut data = fs::read_to_string(path).expect("Unable to read file");
        if let Some(newline_index) = data.find('\n') {
            // Remove the first line (substring after the first '\n')
            data = data[(newline_index + 1)..].to_string();
        }
        if let Some(newline_index) = data.find('\n') {
            let line_size = data[..newline_index].len() + 1;
            println!("line_size: {}", line_size);
            algo(&mut data, line_size);
        }
    }
}
