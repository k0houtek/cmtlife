mod termdraw;

use std::{thread, time};

fn main() {
    let mut matrix: [[usize; 10]; 10] = [[0,0,1,0,0,0,0,0,0,0],
                                         [1,0,1,0,0,0,0,0,0,0],
                                         [0,1,1,0,0,0,0,0,0,0],
                                         [0,0,0,0,0,0,0,0,0,0],
                                         [0,0,0,0,0,0,0,0,0,0],
                                         [0,0,0,0,0,0,0,0,0,0],
                                         [0,0,0,0,0,0,0,0,0,0],
                                         [0,0,0,0,0,0,0,0,0,0],
                                         [0,0,0,0,0,0,0,0,0,0],
                                         [0,0,0,0,0,0,0,0,0,0]];

    let mut second_matrix = matrix.clone();

    loop {
        for x in 0..9 {
            for y in 0..9 {
                termdraw::set_position(x,y);
                termdraw::fill(matrix[y][x] != 0);

                let mut neighbors: u8 = 0;
                if x != 0 && y != 0 && matrix[y-1][x-1] == 1 { neighbors += 1};
                if x != 0 && matrix[y][x-1] == 1 { neighbors += 1};
                if x != 0 && y != 9 && matrix[y+1][x-1] == 1 { neighbors += 1};
                if y != 0 && matrix[y-1][x] == 1 { neighbors += 1};
                if y != 9 && matrix[y+1][x] == 1 { neighbors += 1};
                if x != 9 && y != 0 && matrix[y-1][x+1] == 1 { neighbors += 1};
                if x != 9 && matrix[y][x+1] == 1 { neighbors += 1};
                if x != 9 && y != 9 && matrix[y+1][x+1] == 1 { neighbors += 1};
      
                if matrix[y][x] == 1 {
                    if !(neighbors == 2 || neighbors == 3) { second_matrix [y][x] = 0}
                } else {
                    if neighbors == 3 { second_matrix[y][x] = 1}
                }
            }
        }

        matrix = second_matrix;

        for x in 0..9 {
            for y in 0..9 {
                termdraw::set_position(x,y);
                termdraw::fill(matrix[y][x] != 0);
            }
        }

        let one_second = time::Duration::from_millis(1000);
        thread::sleep(one_second);

    }
}