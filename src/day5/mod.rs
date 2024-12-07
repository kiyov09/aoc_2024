use std::{collections::HashMap, fs::read_to_string};

const PATH: &str = "./inputs/day5/input.txt";

/// turn the rule lines `X|Y` into a tuple of (X, Y)
fn process_ordering_rule(line: &str) -> (u32, u32) {
    line.split_once('|')
        // yeah yeah, unwrap. But we know ther's will be only numbers there
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .unwrap()
}

fn is_valid_page_order(nums: &[u32], graph: &HashMap<u32, Vec<u32>>) -> bool {
    for (i, n) in nums.iter().enumerate() {
        let req = graph.get(n);

        // if this page does not depend on any other, we can skip it.
        // It can literally be anywhere
        if req.is_none() {
            continue;
        }

        let req = req.expect("we know it's Some");

        // Check that all the dependencies are after this page
        // The ones not in `nums` are not relevant, flap_map will just skip them
        if req
            .iter()
            .flat_map(|dep| nums.iter().position(|i| i == dep))
            .all(|pos| pos > i)
        {
            continue;
        }

        // at this point we know there's a dependency that is happening before this page
        // so the whole line is not valid
        return false;
    }

    true
}

// given a line of comma separated numbers, turn it into a vec of numbers
fn nums_from_line(line: &str) -> Vec<u32> {
    line.split(',')
        .map(|n| str::parse(n).unwrap())
        // we reverse it to analyze the pages from the ones that need to happen last
        // to the ones that can happen first
        .rev()
        .collect()
}

fn middle_element(nums: Vec<u32>) -> u32 {
    *nums.get(nums.len() / 2).unwrap()
}

pub fn task_1() {
    let data = read_to_string(PATH).expect("can't read input data");

    let mut input_lines = data.lines();
    let mut graph = HashMap::new();

    // consume the lines till the first empty one
    // and build the graph of dependencies
    while let Some(line) = input_lines.by_ref().next() {
        if line.is_empty() {
            break;
        }

        let (l, r) = process_ordering_rule(line);

        // we'll map the rule in reverse order. So, in the form X|Y we know that X needs to happen
        // before Y, but we'll store it as Y -> X, so each page will have a list of pages that need
        // to happen before it
        graph
            .entry(r)
            .and_modify(|v: &mut Vec<u32>| v.push(l))
            .or_insert(vec![l]);
    }

    let answer = input_lines
        .map(nums_from_line)
        .filter(|line| is_valid_page_order(line, &graph))
        .map(middle_element)
        .sum::<u32>();

    // this line was added after I got the right answer
    assert_eq!(5732, answer);

    println!("{answer}");
}

fn fix_line(nums: Vec<u32>, graph: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut fixed = vec![nums[nums.len() - 1]];

    for n in nums.iter().rev().skip(1) {
        if let Some(req) = graph.get(n) {
            if let Some(idx) = req
                // From all the dependencies of the current page...
                .iter()
                // ... get their index inside of the `fixed` vec
                .flat_map(|dep| fixed.iter().position(|i| i == dep))
                // ... and from those, the leftmost one
                .min()
            {
                // and insert the current page right before the leftmost dependency
                // they way we grant that the rule is satisfied
                fixed.insert(idx.min(fixed.len()), *n);
                continue;
            }
        }
        fixed.push(*n);
    }

    fixed
}

pub fn task_2() {
    let data = read_to_string(PATH).expect("can't read input data");

    let mut input_lines = data.lines();
    let mut graph = HashMap::new();

    // consume the lines till the first empty one
    // and build the graph of dependencies
    while let Some(line) = input_lines.by_ref().next() {
        if line.is_empty() {
            break;
        }

        let (l, r) = process_ordering_rule(line);

        // we'll map the rule in reverse order. So, in the form X|Y we know that X needs to happen
        // before Y, but we'll store it as Y -> X, so each page will have a list of pages that need
        // to happen before it
        graph
            .entry(r)
            .and_modify(|v: &mut Vec<u32>| v.push(l))
            .or_insert(vec![l]);
    }

    let answer = input_lines
        .map(nums_from_line)
        .filter(|line| !is_valid_page_order(line, &graph))
        .map(|line| fix_line(line, &graph))
        .map(middle_element)
        .sum::<u32>();

    // this line was added after I got the right answer
    assert_eq!(4716, answer);

    println!("{answer}");
}
