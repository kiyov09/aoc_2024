use std::{fs::read_to_string, str::FromStr};

const PATH: &str = "./inputs/day7/input.txt";

enum Ops {
    Add,
    Mul,
    Concat,
}

impl Ops {
    fn do_op(&self, left: u64, right: u64) -> u64 {
        match self {
            Ops::Add => left + right,
            Ops::Mul => left * right,
            Ops::Concat => {
                let mut factor = 1;
                let mut temp = left;

                while temp > 0 {
                    factor *= 10;
                    temp /= 10;
                }

                right * factor + left
            }
        }
    }
}

#[derive(Debug)]
struct Test {
    target: u64,
    nums: Vec<u64>,
}

impl Test {
    fn is_valid(&self, ops: &[Ops]) -> bool {
        fn recurse(nums: &[u64], ops: &[Ops]) -> Vec<u64> {
            if nums.len() == 1 {
                return nums.to_vec();
            }

            recurse(&nums[1..], ops)
                .iter()
                .flat_map(|n| {
                    ops.iter()
                        .map(|ops| ops.do_op(nums[0], *n))
                        .collect::<Vec<_>>()
                })
                .collect()
        }

        let mut rev = self.nums.clone();
        rev.reverse();

        recurse(&rev, ops).iter().any(|n| n == &self.target)
    }
}

impl FromStr for Test {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (target, rest) = s
            .split_once(':')
            .expect("can't split the line by the ':' char");

        let nums = rest
            .split_whitespace()
            .filter_map(|n| n.parse::<u64>().ok())
            .collect();

        Ok(Test {
            target: target
                .parse::<u64>()
                .expect("can't turn targe into a number"),
            nums,
        })
    }
}

pub fn task_1() {
    let input = read_to_string(PATH).expect("can't read input file");

    let answer = input
        .lines()
        .flat_map(|l| l.parse::<Test>().ok())
        .filter(|t| t.is_valid(&[Ops::Add, Ops::Mul]))
        .map(|t| t.target)
        .sum::<u64>();

    assert_eq!(2941973819040, answer);

    println!("{answer}");
}

pub fn task_2() {
    let input = read_to_string(PATH).expect("can't read input file");

    let answer = input
        .lines()
        .flat_map(|l| l.parse::<Test>().ok())
        .filter(|t| t.is_valid(&[Ops::Add, Ops::Mul, Ops::Concat]))
        .map(|t| t.target)
        .sum::<u64>();

    assert_eq!(249943041417600, answer);

    println!("{answer}");
}
