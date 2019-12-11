use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

const nr_computers: usize = 5;

#[derive(Debug)]
struct Computer {
  name: String,
  halted: bool,
  at: i32,
  relative_base: i32,
  nums: HashMap<usize, i32>,
  inp_read: Vec<i32>,
  read_at: usize,
  out_write: Vec<i32>,
  write_at: usize,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Mode {
  Immediate,
  Position,
  Relative,
}

impl Computer {
  fn new(nums_vec: Vec<i32>, name: String) -> Computer {
    let mut nums: HashMap<usize, i32> = HashMap::new();
    for (index, &val) in nums_vec.iter().enumerate() {
      nums.insert(index, val);
    }

    Computer {
      name,
      halted: false,
      at: 0,
      relative_base: 0,
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

    let current = nums.get(&(at as usize)).unwrap() % 100;
    let mut modes = nums.get(&(at as usize)).unwrap() / 100;

    println!("current: {}, modes: {}", current, modes);

    let mut immediate = [Mode::Position; 5];
    for i in 0..5 {
      let something = modes % 10;

      immediate[i] = match something {
        0 => Mode::Position,
        1 => Mode::Immediate,
        2 => Mode::Relative,
        _ => {
          panic!("Should be 0, 1 or 2");
        }
      };
      modes /= 10;
    }

    match current {
      // Addition case
      1 => {
        let outp = nums.get(&((at + 3) as usize)).unwrap();

        let val1 = self.val_at(at + 1, immediate[0]);
        let val2 = self.val_at(at + 2, immediate[1]);

        println!("add: {} * {}", val1, val2);

        let sum = val1 + val2;
        self.nums.insert(*outp as usize, sum);

        self.at += 4;
        return true;
      }

      // Multiplication
      2 => {
        let outp = nums.get(&((at + 3) as usize)).unwrap();

        let val1 = self.val_at(at + 1, immediate[0]);
        let val2 = self.val_at(at + 2, immediate[1]);

        println!("mult: {} * {}", val1, val2);
        let sum = val1 * val2;
        self.nums.insert(*outp as usize, sum);

        self.at += 4;
        return true;
      }

      // Read input
      3 => {
        let outp = nums.get(&((at + 1) as usize)).unwrap();

        // Get input
        // let inp = read_int();
        let inp = inp_read[read_at];
        self.read_at += 1;
        println!("read {}", inp);

        self.nums.insert(*outp as usize, inp);

        self.at += 2;
        return true;
      }

      // Write output
      4 => {
        let outp = self.val_at(at + 1, immediate[0]);

        // let temp = *nums.get(&((at + 1) as usize)).unwrap();
        // println!("temp: {}", temp);
        // let outp = *nums.get(&(temp as usize)).unwrap();
        println!("outp: {}", outp);

        println!("output: {}", outp);
        self.out_write.push(outp);
        // self.write_at should be changed after it is read

        self.at += 2;
        return true;
      }

      // Jump if TRUE
      5 => {
        let val1 = self.val_at(at + 1, immediate[0]);
        let val2 = self.val_at(at + 2, immediate[1]);

        println!("jump true: {}, {}", val1, val2);

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
      6 => {
        let val1 = self.val_at(at + 1, immediate[0]);
        let val2 = self.val_at(at + 2, immediate[1]);

        println!("jump false: {}, {}", val1, val2);

        // Jumps here
        if val1 == 0 {
          self.at = val2;
          return true;
        }

        self.at += 3;
        return true;
      }

      // Less than
      7 => {
        let outp = nums.get(&((at + 3) as usize)).unwrap();

        let val1 = self.val_at(at + 1, immediate[0]);
        let val2 = self.val_at(at + 2, immediate[1]);

        println!("less than: {} < {}", val1, val2);

        let eq = if val1 < val2 { 1 } else { 0 };

        self.nums.insert(*outp as usize, eq);

        self.at += 4;
        return true;
      }

      // Equality check
      8 => {
        let outp = nums.get(&((at + 3) as usize)).unwrap();

        let val1 = self.val_at(at + 1, immediate[0]);
        let val2 = self.val_at(at + 2, immediate[1]);

        println!("equal: {} == {}", val1, val2);

        let eq = if val1 == val2 { 1 } else { 0 };

        self.nums.insert(*outp as usize, eq);

        self.at += 4;
        return true;
      }

      // Change relative base
      9 => {
        let val1 = self.val_at(at + 1, immediate[0]);

        println!("change relative base {}", val1);
        self.relative_base += val1;

        self.at += 2;
        return true;
      }

      // Halt
      99 => {
        self.halted = true;
        return false;
      }

      _ => {
        panic!("Opcode not known {} at {}", current, at);
      }
    }
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

  fn val_at(&self, at: i32, mode: Mode) -> i32 {
    if mode == Mode::Immediate {
      *self.nums.get(&(at as usize)).unwrap_or(&0)
    } else if mode == Mode::Position {
      let temp = *self.nums.get(&(at as usize)).unwrap_or(&0);
      *self.nums.get(&(temp as usize)).unwrap_or(&0)
    } else {
      let temp = *self.nums.get(&(at as usize)).unwrap_or(&0);

      let relative = (self.relative_base + temp) as usize;

      *self.nums.get(&relative).unwrap_or(&0)
    }
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

fn combination_tester(combination: &Vec<i32>, initial_nums: &Vec<i32>) -> i32 {
  let mut nums = initial_nums.to_vec();

  let mut computers: Vec<Computer> = vec![
    Computer::new(initial_nums.to_vec(), String::from("A")),
    Computer::new(initial_nums.to_vec(), String::from("B")),
    Computer::new(initial_nums.to_vec(), String::from("C")),
    Computer::new(initial_nums.to_vec(), String::from("D")),
    Computer::new(initial_nums.to_vec(), String::from("E")),
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

  last_output
}

pub fn main() {
  // Reading part
  let contents = fs::read_to_string("input.txt").expect("File couldn't be read");
  let initialNums: Vec<i32> = contents
    .split(",")
    .map(|num| num.parse().unwrap())
    .collect();

  let mut comp = Computer::new(initialNums.to_vec(), String::from("A"));

  while !comp.halted {
    comp.next();
  }

  loop {
    let resp = comp.read_response();

    match resp {
      Some(val) => {
        print!("{}, ", val);
      }
      None => {
        break;
      }
    }
  }

  if false {
    let mut max_output_signal = 0;

    for i0 in 5..10 {
      for i1 in 5..10 {
        for i2 in 5..10 {
          for i3 in 5..10 {
            for i4 in 5..10 {
              let combination: Vec<i32> = vec![i0, i1, i2, i3, i4];
              if is_valid_permutation(&combination) {
                let current = combination_tester(&combination, &initialNums);
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
