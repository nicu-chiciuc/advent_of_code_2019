use num::integer::lcm;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

const moons: usize = 4;

// First is location, then is speeds
type Moons = ([Vector; moons], [Vector; moons]);

type MoonSlice = ([i32; moons], [i32; moons]);

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
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

fn step_slice(state: &mut MoonSlice) {
  let (locations, speeds) = state;
  let mut index = 0;
  while index < 3 {
    let mut j = index + 1;
    while j < 4 {
      if locations[index] < locations[j] {
        speeds[index] += 1;
        speeds[j] -= 1;
      } else if locations[index] > locations[j] {
        speeds[index] -= 1;
        speeds[j] += 1;
      }

      j += 1;
    }

    index += 1;
  }

  let mut i = 0;
  while i < 4 {
    locations[i] += speeds[i];

    i += 1;
  }
}

fn step(state: &mut Moons) {
  let (locations, speeds) = state;

  for index in 0..3 {
    for j in (index + 1)..4 {
      if locations[index].x < locations[j].x {
        speeds[index].x += 1;
        speeds[j].x -= 1;
      } else if locations[index].x > locations[j].x {
        speeds[index].x -= 1;
        speeds[j].x += 1;
      }

      if locations[index].y < locations[j].y {
        speeds[index].y += 1;
        speeds[j].y -= 1;
      } else if locations[index].y > locations[j].y {
        speeds[index].y -= 1;
        speeds[j].y += 1;
      }

      if locations[index].z < locations[j].z {
        speeds[index].z += 1;
        speeds[j].z -= 1;
      } else if locations[index].z > locations[j].z {
        speeds[index].z -= 1;
        speeds[j].z += 1;
      }
    }
  }

  for i in 0..4 {
    locations[i].add(&speeds[i]);
  }

  // print_vectors(&locations);
  // print_vectors(&speeds);
}

fn to_hash(locations: &Vec<Vector>, speeds: &Vec<Vector>) -> String {
  let mut outs = String::new();

  for (index, loc) in locations.iter().enumerate() {
    let sline = format!("loc:{:?}, speed:{:?}\n", loc, speeds[index]);
    outs.push_str(&sline);
  }

  outs
}

fn slice_x(state: &Moons) -> MoonSlice {
  let (locations, speeds) = state;
  let mut slice: MoonSlice = ([0; moons], [0; moons]);

  for i in 0..4 {
    slice.0[i] = locations[i].x;
    slice.1[i] = speeds[i].x;
  }

  slice
}

fn slice_y(state: &Moons) -> MoonSlice {
  let (locations, speeds) = state;
  let mut slice: MoonSlice = ([0; moons], [0; moons]);

  for i in 0..4 {
    slice.0[i] = locations[i].y;
    slice.1[i] = speeds[i].y;
  }

  slice
}

fn slice_z(state: &Moons) -> MoonSlice {
  let (locations, speeds) = state;
  let mut slice: MoonSlice = ([0; moons], [0; moons]);

  for i in 0..4 {
    slice.0[i] = locations[i].z;
    slice.1[i] = speeds[i].z;
  }

  slice
}

fn find_rep_slice(state: MoonSlice) -> i128 {
  let mut state_R = state;
  let mut previous: HashMap<MoonSlice, i64> = HashMap::new();
  previous.insert(state_R, 0);
  // previous.insert(to_hash(&locations, &speeds));

  let mut i: i64 = 0;
  while i < 1000_000_000 {
    i += 1;

    // println!("{:?}", state_R);
    step_slice(&mut state_R);

    if previous.contains_key(&state_R) {
      let prev_loc = previous.get(&state_R).unwrap();
      println!("{:?}", state_R);

      println!("Fund repetition at {}, prev: {}", i, prev_loc);

      return (i - prev_loc) as i128;
    }

    previous.insert(state_R, i);
  }

  panic!("i exceeded the threshold");
}

pub fn main() {
  let contents = fs::read_to_string("input.txt").expect("File couldn't be read");

  let mut locations = [Vector::new(); moons];
  let mut speeds = [Vector::new(); moons];
  for (index, line) in contents.lines().enumerate() {
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

    locations[index] = location;
  }

  let mut state: Moons = (locations, speeds);
  let mut state_x = slice_x(&state);
  let mut state_y = slice_y(&state);
  let mut state_z = slice_z(&state);

  let mut state_R = state_y;

  let diff_x = find_rep_slice(state_x);
  let diff_y = find_rep_slice(state_y);
  let diff_z = find_rep_slice(state_z);

  let val = lcm(lcm(diff_x, diff_y), diff_z);

  println!("{}", val);

  // let mut previous: HashMap<MoonSlice, i64> = HashMap::new();
  // previous.insert(state_R, 0);
  // // previous.insert(to_hash(&locations, &speeds));

  // let mut i: i64 = 0;
  // while i < 1000_000_000 {
  //   i += 1;

  //   // println!("{:?}", state_R);
  //   step_slice(&mut state_R);

  //   if previous.contains_key(&state_R) {
  //     let prev_loc = previous.get(&state_R).unwrap();
  //     println!("{:?}", state_R);

  //     println!("Fund repetition at {}, prev: {}", i, prev_loc);
  //     break;
  //   }

  //   previous.insert(state_R, i);
  // }

  // let e = energy(&locations, &speeds);

  // println!("{}", e);
}
