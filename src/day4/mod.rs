use std::{fs::read_to_string, iter};

const PATH: &str = "./inputs/day4/input.txt";

fn count_xmas(data: &[Vec<char>], row: usize, col: usize) -> usize {
    let deltas = [
        (0, -1),  // go back
        (0, 1),   // go forward
        (-1, 0),  // go up
        (1, 0),   // go down
        (-1, -1), // diagonal left up
        (1, -1),  // diagonal right up
        (-1, 1),  // diagonal left down
        (1, 1),   // diagonal right down
    ];

    deltas
        .iter()
        .filter(|d| {
            iter::repeat(d)
                .enumerate()
                .take(3)
                .map(|(idx, d)| (idx + 1, d))
                .filter_map(|(idx, d)| {
                    let delta_row = idx as i32 * d.0;
                    let delta_col = idx as i32 * d.1;

                    let new_row = row as i32 + delta_row;
                    let new_col = col as i32 + delta_col;

                    if new_row.is_negative() || new_col.is_negative() {
                        return None;
                    }

                    Some((new_row as usize, new_col as usize))
                })
                .filter_map(|(r, c)| data.get(r)?.get(c))
                .zip("MAS".chars())
                .filter(|(a, b)| *a == b)
                .count()
                == 3
        })
        .count()
}

fn get_input_matrix() -> Vec<Vec<char>> {
    let input = read_to_string(PATH).expect("can't read the input");
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn task_1() {
    let data = get_input_matrix();

    let mut count = 0;

    for row in 0..data.len() {
        for col in 0..data[0].len() {
            if data[row][col] == 'X' {
                count += count_xmas(&data, row, col);
            }
        }
    }

    // this line was added after I got right the answer
    assert_eq!(2571, count);

    println!("{count}");
}

pub fn task_2() {
    let data = get_input_matrix();

    // The possible combinations for getting two MAS cross
    // As we can see, there are 4 possible 3x3 matrices

    // M . M  |  M . S  |  S . S  |  S . M
    // . A .  |  . A .  |  . A .  |  . A .
    // S . S  |  M . S  |  M . M  |  S . M

    // Remove the dots of each group from above and join the whole group
    // into an array of 5 elements
    let possibles = [
        ['M', 'M', 'A', 'S', 'S'],
        ['M', 'S', 'A', 'M', 'S'],
        ['S', 'S', 'A', 'M', 'M'],
        ['S', 'M', 'A', 'S', 'M'],
    ];

    let mut count = 0;

    // So we can iterate over each 3x3 view of the big matrix
    for row in 0..data.len() - 2 {
        for col in 0..data[0].len() - 2 {
            // but we only need to check the 5 elements of each view
            // so we pick the ones we care about
            let view = [
                data[row][col],
                data[row][col + 2],
                data[row + 1][col + 1],
                data[row + 2][col],
                data[row + 2][col + 2],
            ];

            // if that matches any of the possible combinations we have an x-mas
            if possibles.iter().any(|p| *p == view) {
                count += 1;
            }
        }
    }

    // this line was added after I got right the answer
    assert_eq!(1992, count);

    println!("{count}");
}
