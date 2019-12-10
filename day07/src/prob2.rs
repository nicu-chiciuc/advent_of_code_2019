use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

#[derive(Debug)]
struct Computer {
  name: String,
  halted: bool,
  at: i32,
  nums: Vec<i32>,
  inp_read: Vec<i32>,
  read_at: usize,
  out_write: Vec<i32>,
  write_at: usize,
}

impl Computer {
  fn new(nums: Vec<i32>, name: String) -> Computer {
    Computer {
      name,
      halted: false,
      at: 0,
      nums: nums,
      inp_read: vec![],
      read_at: 0,
      out_write: vec![],
      write_at: 0,
    }
  }

  // Returns true if can continue
  // false if halts
  fn next(&mut self) -> bool {
    let at = self.at;
    let nums = &self.nums;
    let inp_read = &self.inp_read;
    let read_at = self.read_at;

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
      self.nums[outp as usize] = sum;

      self.at += 4;
      return true;
    }

    // Multiplication
    if current == 2 {
      let outp = nums[(at + 3) as usize];

      let val1 = val_at(&nums, at + 1, immediate[0]);
      let val2 = val_at(&nums, at + 2, immediate[1]);

      // println!("mult: {} * {}", val1, val2);
      let sum = val1 * val2;
      self.nums[outp as usize] = sum;

      self.at += 4;
      return true;
    }

    // Read input
    if current == 3 {
      let outp = nums[(at + 1) as usize];

      // Get input
      // let inp = read_int();
      let inp = inp_read[read_at];
      self.read_at += 1;
      // println!("read {}", inp);

      self.nums[outp as usize] = inp;

      self.at += 2;
      return true;
    }

    // Write output
    if current == 4 {
      let outp = nums[nums[(at + 1) as usize] as usize];

      // println!("output: {}", outp);
      self.out_write.push(outp);
      // self.write_at should be changed after it is read

      self.at += 2;
      return true;
    }

    // Jump if TRUE
    if current == 5 {
      let val1 = val_at(&nums, at + 1, immediate[0]);
      let val2 = val_at(&nums, at + 2, immediate[1]);

      // println!("jump true: {}, {}", val1, val2);

      // Jumps here
      if val1 != 0 {
        self.at = val2;
        return true;
      }

      // output
      self.at += 3;
      return true;
    }

    // Jump if TRUE
    if current == 6 {
      let val1 = val_at(&nums, at + 1, immediate[0]);
      let val2 = val_at(&nums, at + 2, immediate[1]);

      // println!("jump false: {}, {}", val1, val2);

      // Jumps here
      if val1 == 0 {
        self.at = val2;
        return true;
      }

      self.at += 3;
      return true;
    }

    // Less than
    if current == 7 {
      let outp = nums[(at + 3) as usize];

      let val1 = val_at(&nums, at + 1, immediate[0]);
      let val2 = val_at(&nums, at + 2, immediate[1]);

      // println!("less than: {} < {}", val1, val2);

      let eq = if val1 < val2 { 1 } else { 0 };

      self.nums[outp as usize] = eq;

      self.at += 4;
      return true;
    }

    // Equality check
    if current == 8 {
      let outp = nums[(at + 3) as usize];

      let val1 = val_at(&nums, at + 1, immediate[0]);
      let val2 = val_at(&nums, at + 2, immediate[1]);

      // println!("equal: {} == {}", val1, val2);

      let eq = if val1 == val2 { 1 } else { 0 };

      self.nums[outp as usize] = eq;

      self.at += 4;
      return true;
    }

    if current == 99 {
      return false;
    }

    panic!("Opcode not known {} at {}", current, at);
  }

  fn add_to_read(&mut self, val: i32) {
    self.inp_read.push(val);
  }

  fn new_response(&self) -> usize {
    self.out_write.len() - self.write_at
  }

  fn read_response(&mut self) -> Option<i32> {
    if self.out_write.len() > self.write_at {
      self.write_at += 1;

      // println!("Attempting to extract value at {}", self.write_at - 1);

      return Some(self.out_write[self.write_at - 1]);
    }

    None
  }

  fn next_until_output(&mut self) -> Option<i32> {
    loop {
      let running = self.next();

      let resp = self.read_response();
      if let Some(val) = resp {
        // println!("{}: Got output {}", self.name, val);

        return Some(val);
      }

      if !running {
        // println!("Comp {} halted", self.name);
        self.halted = true;
        return None;
      }
    }
  }
}

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
  // println!("input: ");

  io::stdin()
    .read_line(&mut input_text)
    .expect("failed to read from stdin");

  let trimmed = input_text.trim();
  match trimmed.parse::<i32>() {
    Ok(i) => return i,
    Err(..) => panic!("Expected an integer"),
  };
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

  let combination: Vec<i32> = vec![9, 7, 8, 5, 6];

  let nr_computers = 5;

  let mut computers: Vec<Computer> = vec![
    Computer::new(initialNums.to_vec(), String::from("A")),
    Computer::new(initialNums.to_vec(), String::from("B")),
    Computer::new(initialNums.to_vec(), String::from("C")),
    Computer::new(initialNums.to_vec(), String::from("D")),
    Computer::new(initialNums.to_vec(), String::from("E")),
  ];

  // Start by adding all the phase settings
  for comp_i in 0..computers.len() {
    computers[comp_i].add_to_read(combination[comp_i]);
  }

  // First input is 0
  computers[0].add_to_read(0);
  let mut last_output = 0;

  'outer: while !computers[nr_computers - 1].halted {
    // Iterate through each computers getting the input from one and passing it to the other
    for comp_i in 0..computers.len() {
      if computers[comp_i].halted {
        break 'outer;
      }
      let outp = computers[comp_i].next_until_output();
      let next_comp = (comp_i + 1) % nr_computers;

      if let Some(val) = outp {
        computers[next_comp].add_to_read(val);
        last_output = val;
      }
    }
  }

  println!("last output {}", last_output);

  // let current = get_combination_outp(&combination, &initialNums);
  // println!("best: {}", current);

  if false {
    let mut max_output_signal = 0;

    for i0 in 0..5 {
      for i1 in 0..5 {
        for i2 in 0..5 {
          for i3 in 0..5 {
            for i4 in 0..5 {
              let combination: Vec<i32> = vec![i0, i1, i2, i3, i4];
              if is_valid_permutation(&combination) {
                // let current = get_combination_outp(&combination, &initialNums);
                // if current > max_output_signal {
                //   max_output_signal = current;
                // }
              }
            }
          }
        }
      }
    }

    println!("best: {}", max_output_signal);
  }
}
