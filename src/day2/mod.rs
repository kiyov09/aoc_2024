use std::fs::read_to_string;

fn is_valid_delta(a: i32) -> bool {
    (1..=3).contains(&a.abs())
}

const PATH: &str = "./inputs/day2/input.txt";

/// Check if a line is safe, according to the rules described in the problem
fn is_it_safe(line: &[i32]) -> bool {
    let mut ordering = None;

    // Iterater over each consecutive pair of numbers
    for pair in line.windows(2) {
        let (a, b) = (pair[0], pair[1]);

        // Determine the ordering of the pair
        let a_cmp_b = a.cmp(&b);

        let delta = match ordering {
            // The first pair will determine the ordering of the whole sequence
            None => {
                // so store their ordering for later
                ordering = Some(a_cmp_b);
                // and return the difference
                a - b
            }
            // On subsequent pairs, check that the ordering is consistent, and again return
            // the difference
            Some(ord) if ord == a_cmp_b => a - b,
            // If the ordering is inconsistent the line is invalid, so return early
            _ => return false,
        };

        // check this delta is within the valid range
        if !is_valid_delta(delta) {
            return false;
        }
    }

    // At this point everything is valid
    true
}

pub fn task_1() {
    let input = read_to_string(PATH).expect("can't read input");

    let answer = input
        // for each one of the lines
        .lines()
        // Turn the line into a Vec of ints
        .map(|line| {
            line.split_whitespace()
                .map(|n| str::parse(n).unwrap())
                .collect::<Vec<i32>>()
        })
        // process each line, filtering the "unsafe" ones
        .filter(|line| is_it_safe(&line[..]))
        // Count the number of valid lines
        .count();

    // this line was added after I got the right answer
    assert_eq!(624, answer);

    println!("{answer}");
}

pub fn task_2() {
    let input = read_to_string(PATH).expect("can't read input");

    let answer = input
        // for each one of the lines
        .lines()
        // Turn it into a Vec of ints
        .map(|line| {
            line.split_whitespace()
                .map(|n| str::parse(n).unwrap())
                .collect::<Vec<i32>>()
        })
        // process each line, filtering the "unsafe" ones
        .filter(|line| {
            // if the line is safe, return early
            if is_it_safe(line) {
                return true;
            }

            // Else, check how many valid lines we can get by removing each element
            let valids = (0..line.len())
                .filter(|idx| {
                    // split the line at the index
                    let (left, right) = line.split_at(*idx);
                    // and join the two parts
                    // (we need to use right[1..] because the right slice includes the value at idx)
                    let joined = [left, &right[1..]].concat();
                    // check if the resulting line (without the value at idx) is safe
                    is_it_safe(&joined[..])
                })
                // and count all the valid lines we got
                .count();

            // if we got at least one valid line we know that we can fix this line
            // by removing one element
            valids >= 1
        })
        // Count the number of valid lines
        .count();

    // // this line was added after I got the right answer
    assert_eq!(658, answer);

    println!("{answer}");
}
