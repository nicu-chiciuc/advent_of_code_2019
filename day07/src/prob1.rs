use std::collections::HashSet;
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

fn process(nums: &mut Vec<i32>, inpRead: &Vec<i32>) -> Vec<i32> {
  let mut at = 0;
  let mut read_at = 0;
  let mut out_write: Vec<i32> = vec![];

  loop {
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

      // println!("add: {} * {}", val1, val2);

      let sum = val1 + val2;
      nums[outp as usize] = sum;

      at += 4;
      continue;
    }

    // Multiplication
    if current == 2 {
      let outp = nums[(at + 3) as usize];

      let val1 = val_at(&nums, at + 1, immediate[0]);
      let val2 = val_at(&nums, at + 2, immediate[1]);

      // println!("mult: {} * {}", val1, val2);
      let sum = val1 * val2;
      nums[outp as usize] = sum;

      at += 4;
      continue;
    }

    // Read input
    if current == 3 {
      let outp = nums[(at + 1) as usize];

      // Get input
      // let inp = read_int();
      let inp = inpRead[read_at];
      read_at += 1;
      // println!("read {}", inp);

      nums[outp as usize] = inp;

      at += 2;
      continue;
    }

    // Write output
    if current == 4 {
      let outp = nums[nums[(at + 1) as usize] as usize];

      // output
      // println!("output: {}", outp);
      out_write.push(outp);

      at += 2;
      continue;
    }

    // Jump if TRUE
    if current == 5 {
      let val1 = val_at(&nums, at + 1, immediate[0]);
      let val2 = val_at(&nums, at + 2, immediate[1]);

      // println!("jump true: {}, {}", val1, val2);

      // Jumps here
      if val1 != 0 {
        at = val2;
        continue;
      }

      // output
      at += 3;
      continue;
    }

    // Jump if TRUE
    if current == 6 {
      let val1 = val_at(&nums, at + 1, immediate[0]);
      let val2 = val_at(&nums, at + 2, immediate[1]);

      // println!("jump false: {}, {}", val1, val2);

      // Jumps here
      if val1 == 0 {
        at = val2;
        continue;
      }

      at += 3;
      continue;
    }

    // Less than
    if current == 7 {
      let outp = nums[(at + 3) as usize];

      let val1 = val_at(&nums, at + 1, immediate[0]);
      let val2 = val_at(&nums, at + 2, immediate[1]);

      // println!("less than: {} < {}", val1, val2);

      let eq = if val1 < val2 { 1 } else { 0 };

      nums[outp as usize] = eq;

      at += 4;
      continue;
    }

    // Equality check
    if current == 8 {
      let outp = nums[(at + 3) as usize];

      let val1 = val_at(&nums, at + 1, immediate[0]);
      let val2 = val_at(&nums, at + 2, immediate[1]);

      // println!("equal: {} == {}", val1, val2);

      let eq = if val1 == val2 { 1 } else { 0 };

      nums[outp as usize] = eq;

      at += 4;
      continue;
    }

    if current == 99 {
      break;
    }

    panic!("Opcode not known {} at {}", current, at);
  }

  out_write
}

fn get_combination_outp(comb: &Vec<i32>, initial_nums: &Vec<i32>) -> i32 {
  let mut nums = initial_nums.to_vec();
  let mut last_input = 0;

  for i in comb.iter() {
    let inp_reads: Vec<i32> = vec![*i, last_input];

    let output = process(&mut nums, &inp_reads);

    if output.len() > 1 {
      println!("Expected output length to always be 1");
    }

    last_input = output[0];
  }

  for i in comb.iter() {
    print!("{},", i)
  }
  println!(" -> {}", last_input);

  last_input
}

fn is_valid_permutation(comb: &Vec<i32>) -> bool {
  let mut set: HashSet<i32> = HashSet::new();
  for i in comb.iter() {
    set.insert(*i);
  }

  set.len() == comb.len()
}

pub fn main() {
  // Reading part
  let contents = fs::read_to_string("input.txt").expect("File couldn't be read");
  let initialNums: Vec<i32> = contents
    .split(",")
    .map(|num| num.parse().unwrap())
    .collect();

  // let combination: Vec<i32> = vec![1, 0, 4, 3, 2];

  // let current = get_combination_outp(&combination, &initialNums);
  // println!("best: {}", current);

  if true {
    let mut max_output_signal = 0;

    for i0 in 0..5 {
      for i1 in 0..5 {
        for i2 in 0..5 {
          for i3 in 0..5 {
            for i4 in 0..5 {
              let combination: Vec<i32> = vec![i0, i1, i2, i3, i4];
              if is_valid_permutation(&combination) {
                let current = get_combination_outp(&combination, &initialNums);
                if current > max_output_signal {
                  max_output_signal = current;
                }
              }
            }
          }
        }
      }
    }

    println!("best: {}", max_output_signal);
  }
}
