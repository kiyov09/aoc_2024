use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const PATH: &str = "./inputs/day8/input.txt";

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct AntennaPos(isize, isize);

impl From<(isize, isize)> for AntennaPos {
    fn from((row, col): (isize, isize)) -> Self {
        AntennaPos(row, col)
    }
}

impl AntennaPos {
    fn cal_antinodes(&self, other: &AntennaPos) -> (AntennaPos, AntennaPos) {
        let diff_row = self.0 - other.0;
        let diff_col = self.1 - other.1;

        let self_anti = (self.0 + diff_row, self.1 + diff_col);
        let other_anti = (other.0 - diff_row, other.1 - diff_col);

        (self_anti.into(), other_anti.into())
    }

    fn cal_all_antinodes(
        &self,
        other: &AntennaPos,
        row_len: isize,
        col_len: isize,
    ) -> Vec<AntennaPos> {
        let diff_row = self.0 - other.0;
        let diff_col = self.1 - other.1;

        let mut antis = vec![];

        let mut self_anti = (self.0 + diff_row, self.1 + diff_col).into();
        loop {
            if !inside(&self_anti, row_len, col_len) {
                break;
            }
            antis.push(self_anti);
            self_anti = (self_anti.0 + diff_row, self_anti.1 + diff_col).into();
        }

        let mut other_anti = (other.0 - diff_row, other.1 - diff_col).into();
        loop {
            if !inside(&other_anti, row_len, col_len) {
                break;
            }

            antis.push(other_anti);
            other_anti = (other_anti.0 - diff_row, other_anti.1 - diff_col).into();
        }

        antis
    }
}

/// Check if the antenna is inside the map based on the row and col length provided
fn inside(antenna: &AntennaPos, row_len: isize, col_len: isize) -> bool {
    (0..row_len).contains(&antenna.0) && (0..col_len).contains(&antenna.1)
}

struct Data {
    freqs: HashMap<char, Vec<AntennaPos>>,
    row_len: isize,
    col_len: isize,
}

fn common() -> Data {
    let input = read_to_string(PATH).expect("can't read input file");

    let mut row_len = 0;
    let mut col_len = 0;

    // Only antennas of the same frequency can interfere with each other
    // to create antinodes. So we'll store the antennas by frequency
    // in a HashMap, where the key is the frequency and the value is a
    // Vec with the positions of the antennas (AntennaPos)
    let mut freqs = HashMap::new();

    input
        // for each line (row) in the map
        .lines()
        // `enumerate` to have the row #
        .enumerate()
        // start populating the freqs map
        .fold(&mut freqs, |acc, (row, line)| {
            // Calculate the row and col length to be used later
            // row_len will be the same for all rows (lines)
            row_len = line.len();
            // col_len will be the number of rows (lines)
            col_len += 1;

            // for each char in the line
            line.chars()
                // enumerate to have the col #
                .enumerate()
                // filter out the empty spaces. Note filter needs to happen after enumerate,
                // otherwise the col # will be off
                .filter(|(_, c)| *c != '.')
                // for each of the non dot chars
                .for_each(|(col, c)| {
                    // add the position of the antenna to the freqs map
                    // under the key of the frequency. In this case the frequency
                    // is the char itself
                    acc.entry(c)
                        .and_modify(|v: &mut Vec<_>| v.push(AntennaPos(row as isize, col as isize)))
                        .or_insert(vec![AntennaPos(row as isize, col as isize)]);
                });

            acc
        });

    Data {
        freqs,
        row_len: row_len as isize,
        col_len: col_len as isize,
    }
}

pub fn task_1() {
    let Data {
        freqs,
        row_len,
        col_len,
    } = common();

    // We'll store the antinodes in a HashSet to avoid duplicates
    let mut antinodes = HashSet::new();

    // For each frequencies
    for antennas in freqs.values() {
        // calculate the antinodes for each pair of antennas
        for a in antennas {
            antennas.iter().filter(|a_| *a_ != a).for_each(|a_| {
                let (x, y) = a.cal_antinodes(a_);
                antinodes.insert(x);
                antinodes.insert(y);
            })
        }
    }

    // Count the antinodes that are inside the map
    let no_antinodes = antinodes
        .iter()
        .filter(|anti| inside(anti, row_len, col_len))
        .count();

    // this line was added after I got the right answer
    assert_eq!(318, no_antinodes);

    println!("{}", no_antinodes);
}

pub fn task_2() {
    let Data {
        freqs,
        row_len,
        col_len,
    } = common();

    // We'll store the antinodes in a HashSet to avoid duplicates
    let mut antinodes = HashSet::new();

    // For each frequencies
    for antennas in freqs.values() {
        // calculate the antinodes for each pair of antennas
        for a in antennas {
            antennas.iter().filter(|a_| *a_ != a).for_each(|a_| {
                // This time we'll get more than 2 antinodes for each pair of antennas
                a.cal_all_antinodes(a_, row_len, col_len)
                    .into_iter()
                    .for_each(|anti| {
                        antinodes.insert(anti);
                    });
            });

            // If this is not the only antenna of this frequency
            // we'll add the antinodes of the antenna with itself
            if antennas.len() > 1 {
                antinodes.insert(*a);
            }
        }
    }

    // Count the antinodes that are inside the map
    let no_antinodes = antinodes
        .iter()
        .filter(|anti| inside(anti, row_len, col_len))
        .count();

    // this line was added after I got the right answer
    assert_eq!(1126, no_antinodes);

    println!("{}", no_antinodes);
}
