use std::{fs::read_to_string, iter};

const PATH: &str = "./inputs/day9/input.txt";

#[derive(Debug, Clone, Copy)]
enum Block {
    File { size: u32, id: usize },
    Empty { size: u32 },
}

impl Block {
    fn size(&self) -> u32 {
        match self {
            Block::File { size, id: _ } => *size,
            Block::Empty { size } => *size,
        }
    }
}

fn common() -> Vec<Block> {
    let input = read_to_string(PATH).expect("can't read input file");

    input
        .chars()
        .enumerate()
        .flat_map(|(idx, c)| c.to_digit(10).map(|c| (idx, c)))
        .map(|(idx, n)| {
            if idx % 2 == 0 {
                Block::File {
                    size: n,
                    id: (idx / 2),
                }
            } else {
                Block::Empty { size: n }
            }
        })
        .collect::<Vec<_>>()
}

fn get_checksum(data: &[Block]) -> usize {
    data.iter()
        .flat_map(|b| match b {
            Block::File { size, id } => iter::repeat(id).take(*size as usize),
            Block::Empty { size } => iter::repeat(&0_usize).take(*size as usize),
        })
        .enumerate()
        .map(|(id, pos)| pos * id)
        .sum()
}

pub fn task_1() {
    let fs = common();

    let mut data = vec![];
    let mut fs_iter = fs.iter();

    let mut curr: Option<Block> = None;

    while let Some(block) = fs_iter.next() {
        match block {
            Block::File { size: _, id: _ } => data.push(*block),
            Block::Empty { size } => {
                let mut empty_size = *size;

                while empty_size > 0 {
                    let node = match curr {
                        Some(n) => n,
                        None => {
                            if let Some(back) = fs_iter.next_back() {
                                *back
                            } else {
                                break;
                            }
                        }
                    };

                    match node {
                        Block::Empty { size: _ } => continue,
                        Block::File { size, id } => {
                            if empty_size >= size {
                                data.push(Block::File { size, id });
                                curr = None;
                                empty_size -= size;
                            } else {
                                data.push(Block::File {
                                    size: empty_size,
                                    id,
                                });

                                curr = Some(Block::File {
                                    size: size - empty_size,
                                    id,
                                });

                                empty_size = 0;
                            }
                        }
                    };
                }
            }
        };
    }

    // curr will still have something if the last empty space was less than curr's size
    if let Some(curr) = curr {
        match curr {
            Block::File { size, id: _ } if size > 0 => data.push(curr),
            _ => unreachable!(),
        }
    }

    let checksum = get_checksum(&data);

    // this line was added after I got the right answer
    // assert_eq!(6519155389266, checksum);

    println!("{checksum}");
}

pub fn task_2() {
    let mut fs = common();

    let mut ptr = fs.len() - 1;

    while ptr > 0 {
        let block = fs[ptr];
        match block {
            Block::File { size, id: _ } => {
                // find the index of the leftmost empty block that can fit the `block`
                let empty_b_idx = fs.iter().position(
                    |b| matches!(b, Block::Empty { size: empty_size } if empty_size >= &size),
                );

                // this block doesn't fix any empty space, or the empty space is at the right of
                // the block
                if empty_b_idx.is_none() || empty_b_idx.unwrap() >= ptr {
                    ptr -= 1;
                    continue;
                }

                let empty_b_idx = empty_b_idx.unwrap();
                let empty_b = fs[empty_b_idx];

                // if the empty block has the same size as the block, just swap them
                if empty_b.size() == size {
                    fs.swap(empty_b_idx, ptr);
                    continue; // next time on this pos will be empty
                }

                // if not, split the empty block in two parts
                let empty_left = Block::Empty { size };
                let empty_right = Block::Empty {
                    size: empty_b.size() - size,
                };

                // put the left part in the Block::File spot
                fs[ptr] = empty_left;

                // the Block::File in the empty spot
                fs[empty_b_idx] = block;
                // and insert the remaining empty block in the next position
                fs.insert(empty_b_idx + 1, empty_right);
            }
            Block::Empty { size: _ } => {
                ptr -= 1;
                continue;
            }
        }
    }

    let checksum = get_checksum(&fs);

    // this line was added after I got the right answer
    assert_eq!(6547228115826, checksum);

    println!("{checksum}");
}
