use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

use crate::computer::Computer;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
  x: i32,
  y: i32,
}

fn print_vec(vec: &Vec<i64>) {
  for i in vec.iter() {
    print!("{} ", i);
  }
  //println!();
}

fn is_valid_permutation(comb: &Vec<i64>) -> bool {
  let mut set: HashSet<i64> = HashSet::new();
  for i in comb.iter() {
    set.insert(*i);
  }

  set.len() == comb.len()
}

fn move_point_in_direction(p: &mut Point, direction: i64) {
  match direction {
    0 => {
      p.y += 1;
    }
    1 => {
      p.x += 1;
    }
    2 => {
      p.y -= 1;
    }
    3 => p.x -= 1,
    _ => {
      panic!("unknown direction {}", direction);
    }
  }
}

pub fn main() {
  // Reading part
  let contents = fs::read_to_string("input.txt").expect("File couldn't be read");
  let initialNums: Vec<i64> = contents
    .split(",")
    .map(|num| num.parse().unwrap())
    .collect();

  let mut comp = Computer::new(initialNums.to_vec(), String::from("A"));

  // 1 -> white
  // 0 -> black
  let mut color_white: HashMap<Point, i64> = HashMap::new();
  let mut current = Point { x: 0, y: 0 };

  // 0 -> up
  // 1 -> right
  // 2 -> down
  // 3 -> left
  let mut current_direction = 0;

  //println!("------");
  while !comp.halted {
    // Get current color
    let current_color = color_white.get(&current).cloned().unwrap_or(0);

    let next_instruction = current_color;

    comp.add_to_read(next_instruction);

    let next_color_op = comp.next_until_output();
    let next_direction_op = comp.next_until_output();

    if comp.halted {
      break;
    }

    let next_color = next_color_op.unwrap();
    let next_direction = next_direction_op.unwrap();

    println!("color: {}, direction: {}", next_color, next_direction);

    color_white.insert(current, next_color);

    if next_direction == 0 {
      // Move to left
      current_direction = (4 + current_direction - 1) % 4;
      move_point_in_direction(&mut current, current_direction);
    } else if next_direction == 1 {
      current_direction = (4 + current_direction + 1) % 4;
      move_point_in_direction(&mut current, current_direction);
    } else {
      panic!("Direction unknown {}", next_direction);
    }
  }

  println!("{}", color_white.len());
}
