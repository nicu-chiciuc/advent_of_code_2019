use std::env;
use std::fs;
use std::io;

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
    Err(..) => panic!("Expected an integer"),
  };
}

fn process(nums: &mut Vec<i32>, at: i32, inpRead: i32, outWrite: &mut i32) {
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

    println!("add: {} * {}", val1, val2);

    let sum = val1 + val2;
    nums[outp as usize] = sum;

    process(nums, at + 4, inpRead, outWrite);
    return;
  }

  // Multiplication
  if current == 2 {
    let outp = nums[(at + 3) as usize];

    let val1 = val_at(&nums, at + 1, immediate[0]);
    let val2 = val_at(&nums, at + 2, immediate[1]);

    println!("mult: {} * {}", val1, val2);
    let sum = val1 * val2;
    nums[outp as usize] = sum;

    process(nums, at + 4, inpRead, outWrite);
    return;
  }

  // Read input
  if current == 3 {
    let outp = nums[(at + 1) as usize];

    println!("read");

    // Get input
    // let inp = read_int();
    let inp = inpRead;

    nums[outp as usize] = inp;

    process(nums, at + 2, inpRead, outWrite);
    return;
  }

  // Write output
  if current == 4 {
    let outp = nums[nums[(at + 1) as usize] as usize];

    // output
    println!("output: {}", outp);
    *outWrite = outp;

    process(nums, at + 2, inpRead, outWrite);
    return;
  }

  // Jump if TRUE
  if current == 5 {
    let val1 = val_at(&nums, at + 1, immediate[0]);
    let val2 = val_at(&nums, at + 2, immediate[1]);

    println!("jump true: {}, {}", val1, val2);

    // Jumps here
    if val1 != 0 {
      process(nums, val2, inpRead, outWrite);
      return;
    }

    // output

    process(nums, at + 3, inpRead, outWrite);
    return;
  }

  // Jump if TRUE
  if current == 6 {
    let val1 = val_at(&nums, at + 1, immediate[0]);
    let val2 = val_at(&nums, at + 2, immediate[1]);

    println!("jump false: {}, {}", val1, val2);

    // Jumps here
    if val1 == 0 {
      process(nums, val2, inpRead, outWrite);
      return;
    }

    process(nums, at + 3, inpRead, outWrite);
    return;
  }

  // Less than
  if current == 7 {
    let outp = nums[(at + 3) as usize];

    let val1 = val_at(&nums, at + 1, immediate[0]);
    let val2 = val_at(&nums, at + 2, immediate[1]);

    println!("less than: {} < {}", val1, val2);

    let eq = if val1 < val2 { 1 } else { 0 };

    nums[outp as usize] = eq;

    process(nums, at + 4, inpRead, outWrite);
    return;
  }

  // Equality check
  if current == 8 {
    let outp = nums[(at + 3) as usize];

    let val1 = val_at(&nums, at + 1, immediate[0]);
    let val2 = val_at(&nums, at + 2, immediate[1]);

    println!("equal: {} == {}", val1, val2);

    let eq = if val1 == val2 { 1 } else { 0 };

    nums[outp as usize] = eq;

    process(nums, at + 4, inpRead, outWrite);
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

  let mut nums = initialNums.to_vec();
  // nums[1] = 12;
  // nums[2] = 2;

  let mut lastPhaseSetting = 0;

  let combination = vec![4, 3, 2, 1, 0];

  for i in combination.iter() {
    let inpRead = 4;
    let mut outWrite = -1;
    let output = process(&mut nums, 0, inpRead, &mut outWrite);
    println!("{}", outWrite);
  }
}
