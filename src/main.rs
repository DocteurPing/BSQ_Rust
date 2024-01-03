use std::env;
use std::fs;

struct BestSquare{
    index: usize,
    size: usize,
}

fn algo(map: &mut Vec<u8>, line_size: usize) {
    let mut best_square = BestSquare{index: 0, size: 0};
    for i in 0..map.len() {
        if map[i] == b'.' {
            check_square_size(map, &mut best_square, line_size, i);
        }
    }
    put_cross(&mut best_square, map, line_size);
}

fn put_cross(best_square: &mut BestSquare, map: &mut [u8], line_size: usize) {
    for i in 0..best_square.size {
        for j in 0..best_square.size {
            let index = i * line_size + best_square.index + j;
            map[index] = b'X';
        }
    }
    println!("{}", String::from_utf8_lossy(map));
}

fn check_square_size(map: &Vec<u8>, best_square: &mut BestSquare, line_size: usize, index: usize) {
    let mut size = best_square.size + 1;
    loop {
        for i in 0..size {
            for j in 0..size {
                if i * line_size + j + index >= map.len() {
                    return;
                }
                if map[i * line_size + j + index] != b'.' {
                    return;
                }
            }
        }
        best_square.size = size;
        best_square.index = index;
        size += 1;
    }
}

fn main() {
    if let Some(path) = env::args().nth(1) {
        let mut data = fs::read(path).expect("Unable to read file");
        if let Some(newline_index) = data.iter().position(|&x| x == b'\n') {
            // Remove the first line (remove elements up to the first '\n')
            data.drain(..newline_index + 1);
        }
        // Get the size of a line (the first line is the same size as the others)
        let line_size = data.iter().position(|&x| x == b'\n').unwrap_or(data.len()) + 1;
        algo(&mut data, line_size);
    }
}
