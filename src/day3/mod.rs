use core::str;
use std::fs::read_to_string;

const PATH: &str = "./inputs/day3/input.txt";

// Yes, we can improve on this. Later... maybe

pub fn task_1() {
    let program = read_to_string(PATH).expect("can't input file");
    let mut muls = vec![];

    let chars = program.as_bytes();
    let mut ptr = 0;

    while ptr < chars.len() - 4 {
        match &chars[ptr..ptr + 4] {
            // find the word `mul` followed by an open paren
            [b'm', b'u', b'l', b'('] => {
                ptr += 4;

                // next up to 3 numbers
                let end_first_num = &chars[ptr..ptr + 3]
                    .iter()
                    .take_while(|c| c.is_ascii_digit())
                    .count();

                // we don't found a number after the paren
                if *end_first_num == 0 {
                    continue;
                }

                let first_num = str::from_utf8(&chars[ptr..ptr + end_first_num])
                    .unwrap()
                    .parse::<u32>()
                    .expect("we already know they're numbers");

                ptr += end_first_num;

                // no comma after the first number
                if chars[ptr] != b',' {
                    ptr += 1;
                    continue;
                }

                ptr += 1;

                // next up to 3 numbers
                let end_snd_num = &chars[ptr..ptr + 3]
                    .iter()
                    .take_while(|c| c.is_ascii_digit())
                    .count();

                // we don't found a number after the paren
                if *end_snd_num == 0 {
                    continue;
                }

                let snd_num = str::from_utf8(&chars[ptr..ptr + end_snd_num])
                    .unwrap()
                    .parse::<u32>()
                    .expect("we already know they're numbers");

                ptr += end_snd_num;

                if chars[ptr] != b')' {
                    ptr += 1;
                    continue;
                }

                muls.push(first_num * snd_num);

                ptr += 1;
                // comma
                // up to 3 numbers
                // close paren
            }
            // advance the pointer and keep looking
            _ => {
                ptr += 1;
                continue;
            }
        };
    }

    let answer = muls.iter().sum::<u32>();

    // this line was added after I got right the answer
    assert_eq!(164730528, answer);

    println!("{answer}");
}

pub fn task_2() {
    let program = read_to_string(PATH).expect("can't input file");
    let mut muls = vec![];

    let chars = program.as_bytes();
    let mut ptr = 0;

    let mut do_flag = true;

    while ptr < chars.len() - 4 {
        match &chars[ptr..ptr + 4] {
            // find do
            [b'd', b'o', b'(', b')'] => {
                do_flag = true;
                ptr += 4;
            }
            // find don't
            [b'd', b'o', b'n', b'\''] => {
                ptr += 4;
                if chars[ptr..ptr + 3] == [b't', b'(', b')'] {
                    ptr += 3;
                    do_flag = false;
                }
            }
            // find the word `mul` followed by an open paren
            [b'm', b'u', b'l', b'('] if do_flag => {
                ptr += 4;

                // next up to 3 numbers
                let end_first_num = &chars[ptr..ptr + 3]
                    .iter()
                    .take_while(|c| c.is_ascii_digit())
                    .count();

                // we don't found a number after the paren
                if *end_first_num == 0 {
                    continue;
                }

                let first_num = str::from_utf8(&chars[ptr..ptr + end_first_num])
                    .unwrap()
                    .parse::<u32>()
                    .expect("we already know they're numbers");

                ptr += end_first_num;

                // no comma after the first number
                if chars[ptr] != b',' {
                    ptr += 1;
                    continue;
                }

                ptr += 1;

                // next up to 3 numbers
                let end_snd_num = &chars[ptr..ptr + 3]
                    .iter()
                    .take_while(|c| c.is_ascii_digit())
                    .count();

                // we don't found a number after the paren
                if *end_snd_num == 0 {
                    continue;
                }

                let snd_num = str::from_utf8(&chars[ptr..ptr + end_snd_num])
                    .unwrap()
                    .parse::<u32>()
                    .expect("we already know they're numbers");

                ptr += end_snd_num;

                if chars[ptr] != b')' {
                    ptr += 1;
                    continue;
                }

                muls.push(first_num * snd_num);

                ptr += 1;
                // comma
                // up to 3 numbers
                // close paren
            }
            // advance the pointer and keep looking
            _ => {
                ptr += 1;
                continue;
            }
        };
    }

    let answer = muls.iter().sum::<u32>();

    // this line was added after I got right the answer
    assert_eq!(70478672, answer);

    println!("{answer}");
}
