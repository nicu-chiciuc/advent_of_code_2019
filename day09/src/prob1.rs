use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

const nr_computers: i64 = 5;

#[derive(Debug)]
struct Computer {
  name: String,
  halted: bool,
  at: i64,
  relative_base: i64,
  nums: HashMap<i64, i64>,
  inp_read: Vec<i64>,
  read_at: i64,
  out_write: Vec<i64>,
  write_at: i64,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Mode {
  Immediate,
  Position,
  Relative,
}

impl Computer {
  fn new(nums_vec: Vec<i64>, name: String) -> Computer {
    let mut nums: HashMap<i64, i64> = HashMap::new();
    for (index, &val) in nums_vec.iter().enumerate() {
      nums.insert(index as i64, val);
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

    let current = nums.get(&(at as i64)).unwrap() % 100;
    let mut modes = nums.get(&(at as i64)).unwrap() / 100;

    let print_loc = format!(
      "at: {}, current: {}, modes: {}\n\n",
      self.at, current, modes
    );

    let mut mode = [Mode::Position; 5];
    // Extract modes
    for i in 0..5 {
      let something = modes % 10;

      mode[i] = match something {
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
        let val1 = self.val_at(at + 1, mode[0]);
        let val2 = self.val_at(at + 2, mode[1]);
        let outp = self.out_val_at(at + 3, mode[2]);

        let sum = val1 + val2;
        self.nums.insert(outp, sum);

        self.at += 4;

        //println!("add: {} + {}", val1, val2);
        //println!("  {}", print_loc);
        return true;
      }

      // Multiplication
      2 => {
        let val1 = self.val_at(at + 1, mode[0]);
        let val2 = self.val_at(at + 2, mode[1]);
        let outp = self.out_val_at(at + 3, mode[2]);

        //println!("mult: {} * {}", val1, val2);
        //println!("  {}", print_loc);
        let sum = val1 * val2;
        self.nums.insert(outp, sum);

        self.at += 4;
        return true;
      }

      // Read input
      3 => {
        let outp = self.out_val_at(at + 1, mode[0]);

        // Get input
        // let inp = read_int();
        let inp = inp_read[read_at as usize];
        self.read_at += 1;
        //println!("read {}", inp);
        //println!("  {}", print_loc);

        self.nums.insert(outp, inp);

        self.at += 2;
        return true;
      }

      // Write output
      4 => {
        let val1 = self.val_at(at + 1, mode[0]);

        //println!("output: {}", val1);
        //println!("  {}", print_loc);
        self.out_write.push(val1);

        self.at += 2;
        return true;
      }

      // Jump if TRUE
      5 => {
        let val1 = self.val_at(at + 1, mode[0]);
        let val2 = self.val_at(at + 2, mode[1]);

        //println!("jump true: {}, {}", val1, val2);
        //println!("  {}", print_loc);

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
        let val1 = self.val_at(at + 1, mode[0]);
        let val2 = self.val_at(at + 2, mode[1]);

        //println!("jump false: {}, {}", val1, val2);
        //println!("  {}", print_loc);

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
        let val1 = self.val_at(at + 1, mode[0]);
        let val2 = self.val_at(at + 2, mode[1]);
        let outp = self.out_val_at(at + 3, mode[2]);

        //println!("less than: {} < {}", val1, val2);
        //println!("  {}", print_loc);

        let eq = if val1 < val2 { 1 } else { 0 };

        self.nums.insert(outp, eq);

        self.at += 4;
        return true;
      }

      // Equality check
      8 => {
        let val1 = self.val_at(at + 1, mode[0]);
        let val2 = self.val_at(at + 2, mode[1]);
        let outp = self.out_val_at(at + 3, mode[2]);

        //println!("equal: {} == {}", val1, val2);
        //println!("  {}", print_loc);

        let eq = if val1 == val2 { 1 } else { 0 };

        self.nums.insert(outp, eq);

        self.at += 4;
        return true;
      }

      // Change relative base
      9 => {
        let val1 = self.val_at(at + 1, mode[0]);

        self.relative_base += val1;
        self.at += 2;
        //println!(
        //   "change base {}, new base {}, next at {}",
        //   val1, self.relative_base, self.at
        // );
        return true;
      }

      // Halt
      99 => {
        self.halted = true;
        //println!("halt");
        //println!("  {}", print_loc);
        return false;
      }

      _ => {
        panic!("Opcode not known {} at {}", current, at);
      }
    }
  }

  fn add_to_read(&mut self, val: i64) {
    self.inp_read.push(val);
  }

  fn read_response(&mut self) -> Option<i64> {
    if (self.out_write.len() as i64) > self.write_at {
      self.write_at += 1;

      // //println!("Attempting to extract value at {}", self.write_at - 1);

      return Some(self.out_write[(self.write_at - 1) as usize]);
    }

    None
  }

  fn next_until_output(&mut self) -> Option<i64> {
    loop {
      let running = self.next();

      let resp = self.read_response();
      if let Some(val) = resp {
        // //println!("{}: Got output {}", self.name, val);

        return Some(val);
      }

      if !running {
        // //println!("Comp {} halted", self.name);
        self.halted = true;
        return None;
      }
    }
  }

  // Returns the location where it should output
  fn out_val_at(&self, at: i64, mode: Mode) -> i64 {
    match mode {
      Mode::Immediate => {
        panic!("Mode::Immediat doesn't make sens for outputs",);
        // //println!("    immediate: at: {}, value: {}", at, value);
      }
      Mode::Position => {
        let value = *self.nums.get(&(at)).unwrap_or(&0);

        //println!("    position: at: {}, value: {}", at, value);
        value
      }
      Mode::Relative => {
        let temp = *self.nums.get(&at).unwrap_or(&0);

        let relative = self.relative_base + temp;
        let value = relative;

        //println!(
        //   "    relative, at: {}, temp: {}, value: {}, relative: {}, base: {}",
        //   at, temp, value, relative, self.relative_base
        // );
        value
      }
    }
  }

  fn val_at(&self, at: i64, mode: Mode) -> i64 {
    match mode {
      Mode::Immediate => {
        let value = *self.nums.get(&at).unwrap_or(&0);
        //println!("    immediate: at: {}, value: {}", at, value);
        value
      }
      Mode::Position => {
        let temp = *self.nums.get(&(at)).unwrap_or(&0);
        let value = *self.nums.get(&temp).unwrap_or(&0);

        //println!("    position: at: {}, temp: {}, value: {}", at, temp, value);
        value
      }
      Mode::Relative => {
        let temp = *self.nums.get(&at).unwrap_or(&0);

        let relative = self.relative_base + temp;
        let value = *self.nums.get(&relative).unwrap_or(&0);

        //println!(
        //   "    relative, at: {}, temp: {}, value: {}, relative: {}, base: {}",
        //   at, temp, value, relative, self.relative_base
        // );
        value
      }
    }
  }
}

fn print_vec(vec: &Vec<i64>) {
  for i in vec.iter() {
    print!("{} ", i);
  }
  //println!();
}

fn read_int() -> i64 {
  let mut input_text = String::new();
  // //println!("input: ");

  io::stdin()
    .read_line(&mut input_text)
    .expect("failed to read from stdin");

  let trimmed = input_text.trim();
  match trimmed.parse::<i64>() {
    Ok(i) => return i,
    Err(..) => panic!("Expected an integer"),
  };
}

fn is_valid_permutation(comb: &Vec<i64>) -> bool {
  let mut set: HashSet<i64> = HashSet::new();
  for i in comb.iter() {
    set.insert(*i);
  }

  set.len() == comb.len()
}

fn combination_tester(combination: &Vec<i64>, initial_nums: &Vec<i64>) -> i64 {
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

  'outer: while !computers[(nr_computers - 1) as usize].halted {
    // Iterate through each computers getting the input from one and passing it to the other
    for comp_i in 0..computers.len() {
      if computers[comp_i].halted {
        break 'outer;
      }
      let outp = computers[comp_i].next_until_output();
      let next_comp = ((comp_i + 1) as i64) % nr_computers;

      if let Some(val) = outp {
        computers[next_comp as usize].add_to_read(val);
        last_output = val;
      }
    }
  }

  last_output
}

pub fn main() {
  // Reading part
  let contents = fs::read_to_string("input.txt").expect("File couldn't be read");
  let initialNums: Vec<i64> = contents
    .split(",")
    .map(|num| num.parse().unwrap())
    .collect();

  let mut comp = Computer::new(initialNums.to_vec(), String::from("A"));

  // For problem 1 change to "1"
  comp.add_to_read(2);
  while !comp.halted {
    comp.next();
  }

  //println!("------");
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
              let combination: Vec<i64> = vec![i0, i1, i2, i3, i4];
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

    //println!("best: {}", max_output_signal);
  }
}
