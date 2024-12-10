use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const PATH: &str = "./inputs/day10/input.txt";

type Pos = (usize, usize);

/// Check if the antenna is inside the map based on the row and col length provided
fn inside(pos: &Pos, row_len: usize, col_len: usize) -> bool {
    (0..row_len).contains(&pos.0) && (0..col_len).contains(&pos.1)
}

fn find_trailhead(pos: Pos, map: &[Vec<u32>], paths: &mut HashMap<Pos, u32>) -> HashSet<Pos> {
    let val_at_pos = map[pos.0][pos.1];

    // we get to the end. (stop condition)
    // Add it to the hashmap of paths.
    // And return it in a HashSet.
    if val_at_pos == 0 {
        paths
            .entry(pos)
            .and_modify(|v: &mut u32| *v += 1)
            .or_insert(1);

        return HashSet::from_iter([pos]);
    }

    let row_len = map.len();
    let col_len = map[0].len();

    // all possible moves: (left, up, right, down)
    let deltas = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    // and for each on of them
    deltas
        .iter()
        // calculate the new position and filter out the ones that are out of bounds
        .filter_map(|delta| {
            let new_pos = (
                ((pos.0 as isize) + delta.0) as usize,
                ((pos.1 as isize) + delta.1) as usize,
            );
            if inside(&new_pos, row_len, col_len) {
                Some(new_pos)
            } else {
                None
            }
        })
        // for the ones that are in bounds, check if the value is exactly one less than the current
        // value (remember, we're going backwards), and if so, recurse.
        .flat_map(|p| {
            if map[p.0][p.1] != val_at_pos - 1 {
                return None;
            }
            Some(find_trailhead(p, map, paths))
        })
        // flatten all the sets into a single one
        .flatten()
        .collect()
}

fn common() -> (u32, u32) {
    let input = read_to_string(PATH).expect("can't read input file");

    // turn the input into a matrix of nums
    let map = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<_>>>();

    let mut trailheads = HashMap::new();
    let mut hiking_paths = HashMap::new();

    // Instead of starting from a possible trailhead (val 0) and finding how many
    // ends (val 9) we can reach, we'll start from the ends and find how many trailheads
    // we can reach.

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            // for each pos, if the value is 9
            if map[row][col] == 9 {
                find_trailhead((row, col), &map, &mut hiking_paths)
                    .into_iter()
                    .for_each(|th| {
                        // `th` are all the trailheads we can reach from this end.
                        // If we count how many times we reach each trailhead, we'll end up
                        // with the number of ends that trailhead can reach.
                        trailheads
                            .entry(th)
                            .and_modify(|v: &mut u32| *v += 1)
                            .or_insert(1);
                    })
            }
        }
    }

    (trailheads.values().sum(), hiking_paths.values().sum())
}

pub fn task_1() {
    let (answer, _) = common();
    assert_eq!(789, answer);
    println!("{answer}");
}

pub fn task_2() {
    let (_, answer) = common();
    assert_eq!(1735, answer);
    println!("{answer}");
}
