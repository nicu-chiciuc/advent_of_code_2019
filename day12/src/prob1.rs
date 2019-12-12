use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Vector {
  x: i32,
  y: i32,
  z: i32,
}

impl Vector {
  fn new() -> Vector {
    Vector { x: 0, y: 0, z: 0 }
  }

  fn add(&mut self, other: &Vector) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }

  fn energy(&self) -> i32 {
    self.x.abs() + self.y.abs() + self.z.abs()
  }
}

fn print_vectors(vv: &Vec<Vector>) {
  for v in vv.iter() {
    println!("{:?}", v);
  }
  println!();
}

fn energy(locations: &Vec<Vector>, speeds: &Vec<Vector>) -> i32 {
  // Potential energy
  let mut sum = 0;
  for (index, loc) in locations.iter().enumerate() {
    let pot = loc.energy();
    let kin = speeds[index].energy();
    let total = pot * kin;

    sum += total;
  }
  sum
}

fn step(locations: &mut Vec<Vector>, speeds: &mut Vec<Vector>) {
  for (index, loc) in locations.iter().enumerate() {
    let mut gravity = Vector::new();

    for other in locations.iter() {
      if loc == other {
        continue;
      }

      if loc.x < other.x {
        gravity.x += 1;
      } else if loc.x > other.x {
        gravity.x -= 1;
      }

      if loc.y < other.y {
        gravity.y += 1;
      } else if loc.y > other.y {
        gravity.y -= 1;
      }

      if loc.z < other.z {
        gravity.z += 1;
      } else if loc.z > other.z {
        gravity.z -= 1;
      }
    }

    speeds[index].add(&gravity);
  }

  // Change locations
  for (index, loc) in locations.iter_mut().enumerate() {
    loc.add(&speeds[index]);
  }

  print_vectors(&locations);
  print_vectors(&speeds);
}

fn to_hash(locations: &Vec<Vector>, speeds: &Vec<Vector>) -> String {
  let mut outs = String::new();

  for (index, loc) in locations.iter().enumerate() {
    let sline = format!("loc:{:?}, speed:{:?}\n", loc, speeds[index]);
    outs.push_str(&sline);
  }

  outs
}

pub fn main() {
  let contents = fs::read_to_string("input.txt").expect("File couldn't be read");

  let mut locations: Vec<Vector> = vec![];
  let mut speeds: Vec<Vector> = vec![];
  for line in contents.lines() {
    let mut sline = String::from(line);
    sline.pop();
    sline.remove(0);

    let parsed: Vec<i32> = sline
      .split(", ")
      .map(|part| {
        let vec: Vec<&str> = part.split("=").collect();
        let num: i32 = vec[1].parse().unwrap();
        num
      })
      .collect();

    let location = Vector {
      x: parsed[0],
      y: parsed[1],
      z: parsed[2],
    };

    locations.push(location);
    speeds.push(Vector::new());
  }

  let mut previous: HashSet<String> = HashSet::new();
  previous.insert(to_hash(&locations, &speeds));

  for i in 0..3000 {
    step(&mut locations, &mut speeds);
    let hash = to_hash(&locations, &speeds);

    if previous.contains(&hash) {
      println!("Fund repetition at {}", i);
      break;
    }

    previous.insert(hash);
  }

  let e = energy(&locations, &speeds);

  println!("{}", e);
}
