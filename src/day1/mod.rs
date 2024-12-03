use std::{collections::HashMap, fs::read_to_string};

const PATH: &str = "./inputs/day1/input.txt";

fn read_input() -> String {
    read_to_string(PATH).expect("can't read input file")
}

/// Split the input line by 3 spaces and turn each side into a number. Then return a tuple
/// containing both numbers
fn process_line(line: &str) -> (u32, u32) {
    line.split_once("   ")
        .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
        .expect("error processing the input line")
}

pub fn task_1() {
    let input = read_input();

    let (mut left, mut right) = (vec![], vec![]);

    // Process each line and put the number into the corresponding vec
    input.lines().map(process_line).for_each(|(l, r)| {
        left.push(l);
        right.push(r)
    });

    // sort the vecs as we need to process pairs starting from the lesser ones
    left.sort();
    right.sort();

    // using zip, process each pair and then sum up the resulting numbers
    let answer: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    // this line was added after I got the right answer
    assert_eq!(1938424, answer);

    println!("{answer}");
}

pub fn task_2() {
    let input = read_input();

    let (mut left, mut right) = (vec![], HashMap::new());

    // Process each line, turn it into a tuple of numbers
    input.lines().map(process_line).for_each(|(l, r)| {
        // first one goes into a vec
        left.push(l);
        // second one goes into a hashmap to count the number of times it appears
        right
            .entry(r)
            .and_modify(|v: &mut u32| *v += 1)
            .or_insert(1);
    });

    // multiply each number in the left vec by the number of times it appears in the right hashmap
    // and sum up the results
    let answer: u32 = left
        .iter()
        .map(|k| right.get(k).map(|v| k * v).unwrap_or(0))
        .sum();

    // this line was added after I got the right answer
    assert_eq!(22014209, answer);

    println!("{answer}")
}
