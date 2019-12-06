use std::env;
use std::fs;

fn process(nums: &mut Vec<usize>, at: usize) {
    let current = nums[at];

    // Addition case
    if current == 1 {
        let inp1 = nums[at + 1];
        let inp2 = nums[at + 2];
        let outp = nums[at + 3];

        let sum = nums[inp1] + nums[inp2];
        nums[outp] = sum;

        process(nums, at + 4);
        return;
    }

    if current == 2 {
        let inp1 = nums[at + 1];
        let inp2 = nums[at + 2];
        let outp = nums[at + 3];

        let sum = nums[inp1] * nums[inp2];
        nums[outp] = sum;

        process(nums, at + 4);
        return;
    }

    if current == 99 {
        return;
    }

    panic!("Opcode not known {} at {}", current, at);
}

fn main() {
    // Reading part
    let contents = fs::read_to_string("input.txt").expect("File couldn't be read");
    let mut initialNums: Vec<usize> = contents
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut nums = initialNums.to_vec();

            nums[1] = noun;
            nums[2] = verb;

            print!("{},{}: ", noun, verb);

            process(&mut nums, 0);

            if nums[0] == 19690720 {
                println!("Found it!!!!: {}", nums[0]);
                break 'outer;
            } else {
                println!("{}", nums[0]);
            }
        }
    }

    // let mut nums = initialNums.to_vec();

    // nums[1] = 12;
    // nums[2] = 2;

    // let output = process(&mut nums, 0);

    // for num in nums.iter() {
    //     print!("{} ", num);
    // }
    // println!();

    // println!("First num {}", output);

    // let b = calc_rec(100756, 0);
}
