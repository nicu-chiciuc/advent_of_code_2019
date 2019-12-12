use std::collections::HashMap;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Mode {
  Immediate,
  Position,
  Relative,
}

#[derive(Debug)]
pub struct Computer {
  name: String,
  pub halted: bool,
  at: i64,
  relative_base: i64,
  nums: HashMap<i64, i64>,
  inp_read: Vec<i64>,
  read_at: i64,
  out_write: Vec<i64>,
  write_at: i64,
}

impl Computer {
  pub fn new(nums_vec: Vec<i64>, name: String) -> Computer {
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
  pub fn next(&mut self) -> bool {
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

  pub fn add_to_read(&mut self, val: i64) {
    self.inp_read.push(val);
  }

  pub fn read_response(&mut self) -> Option<i64> {
    if (self.out_write.len() as i64) > self.write_at {
      self.write_at += 1;

      // //println!("Attempting to extract value at {}", self.write_at - 1);

      return Some(self.out_write[(self.write_at - 1) as usize]);
    }

    None
  }

  pub fn next_until_output(&mut self) -> Option<i64> {
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
  pub fn out_val_at(&self, at: i64, mode: Mode) -> i64 {
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
