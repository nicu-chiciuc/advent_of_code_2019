use std::env;
use std::io;
use std::fs;

fn val_at(nums: &Vec<i32>, at: i32, immediate: bool) -> i32 {
  if immediate {
    nums[at as usize]
  } else {
    nums[nums[at as usize] as usize]
  }
}

fn print_vec(vec: &Vec<i32>) {
  for i in vec.iter() {
    print!("{} ", i);
  }
  println!();
}

fn read_int() -> i32 {
    let mut input_text = String::new();
    println!("input: ");

    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => return i,
        Err(..) => panic!("Expected an integer")
    };
}

fn process(nums: &mut Vec<i32>, at: i32) {
  let current = nums[at as usize] % 100;
  let mut modes = nums[at as usize] / 100;

  let mut immediate = [false; 5];
  for i in 0..5 {
    let something = modes % 10;
    if something > 1 {
      panic!("Should be 0 or 1");
    }

    immediate[i] = something == 1;
    modes /= 10;
  }

  // Addition case
  if current == 1 {
    let outp = nums[(at + 3) as usize];

    let val1 = val_at(&nums, at + 1, immediate[0]);
    let val2 = val_at(&nums, at + 2, immediate[1]);

    let sum = val1 + val2;
    nums[outp as usize] = sum;

    process(nums, at + 4);
    return;
  }

  if current == 2 {
    let outp = nums[(at + 3) as usize];

    let val1 = val_at(&nums, at + 1, immediate[0]);
    let val2 = val_at(&nums, at + 2, immediate[1]);

    println!("op2: {} * {}", val1, val2);
    
    let sum = val1 * val2;
    nums[outp as usize] = sum;

    process(nums, at + 4);
    return;
  }

  if current == 3 {
    let outp = nums[(at + 1) as usize];

    // Get input
    let inp = read_int();

    nums[outp as usize] = inp;

    process(nums, at + 2);
    return;
  }

  if current == 4 {
    let outp = nums[nums[(at + 1) as usize] as usize];

    // output
    println!("output: {}", outp);

    process(nums, at + 2);
    return;
  }

  if current == 99 {
    return;
  }

  panic!("Opcode not known {} at {}", current, at);
}

pub fn main() {
  // Reading part
  let contents = fs::read_to_string("input.txt").expect("File couldn't be read");
  let mut initialNums: Vec<i32> = contents
    .split(",")
    .map(|num| num.parse().unwrap())
    .collect();

  let prob01 = true;
  let prob02 = false;

  if prob02 {
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
  }

  if prob01 {
    let mut nums = initialNums.to_vec();
    
    // nums[1] = 12;
    // nums[2] = 2;

    let output = process(&mut nums, 0);
    for num in nums.iter() {
      print!("{} ", num);
    }
    println!();

    println!("First num {:?}", output);
  }
}
