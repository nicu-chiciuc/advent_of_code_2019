use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

const width: usize = 25;
const height: usize = 6;
const size: usize = width * height;

fn fewest_0_digits(layers: &Vec<(i32, i32, i32)>) -> (i32, i32, i32) {
  let mut fewest: (i32, i32, i32) = (1000000000, 0, 0);

  for layer in layers.iter() {
    if layer.0 < fewest.0 {
      fewest = layer.clone();
    }
  }

  fewest
}

pub fn main() {
  let full_file = fs::read_to_string("input.txt").expect("File couldn't be read");
  let nums: Vec<u32> = full_file.chars().map(|d| d.to_digit(10).unwrap()).collect();

  let loyer: [i32; size] = [0; size];
  let mut layers: Vec<(i32, i32, i32)> = vec![(0, 0, 0)];

  let mut i: usize = 0;
  let mut layer: usize = 0;

  while i < nums.len() {
    let num = nums[i];

    match num {
      0 => layers[layer].0 += 1,
      1 => layers[layer].1 += 1,
      2 => layers[layer].2 += 1,
      _ => println!("unknown number {}", num),
    }

    let mut layerAt = layers[layer];

    i += 1;
    if (i as u32) % size == 0 {
      layer += 1;

      layers.push((0, 0, 0));
    }
  }
  // Last layer will not be populated
  layers.pop();

  let fewest = fewest_0_digits(&layers);

  let resp = fewest.1 * fewest.2;

  for (index, layer) in layers.iter().enumerate() {
    println!("{}, {:?}", index, layer);
  }

  println!("{}", resp);
}
