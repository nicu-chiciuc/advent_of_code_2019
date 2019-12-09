use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

const width: usize = 25;
const height: usize = 6;
const size: usize = width * height;

fn find_complete_img(layers: &Vec<[u32; size]>) -> [u32; size] {
  let mut outp: [u32; size] = [0; size];

  for i in 0..size {
    // Iterate through each layer
    let mut found = 2;
    for layer in layers.iter() {
      if layer[i] != 2 {
        found = layer[i];
        break;
      }
    }

    outp[i] = found;
  }

  outp
}

fn print_layer(layer: &[u32; size]) {
  println!("---");
  for y in 0..height {
    for x in 0..width {
      print!("{}", layer[x + y * width])
    }
    println!("");
  }
}

pub fn main() {
  let full_file = fs::read_to_string("input.txt").expect("File couldn't be read");
  let nums: Vec<u32> = full_file.chars().map(|d| d.to_digit(10).unwrap()).collect();

  let loyer: [u32; size] = [0; size];
  let mut layers: Vec<[u32; size]> = vec![[0; size]];

  let mut i: usize = 0;
  let mut layer: usize = 0;

  while i < nums.len() {
    let num = nums[i];

    let layer_i = i % size;
    layers[layer][layer_i] = num;

    i += 1;
    if i % size == 0 {
      layer += 1;

      layers.push([0; size]);
    }
  }
  // Last layer will not be populated
  layers.pop();

  let final_img = find_complete_img(&layers);

  for layer in layers.iter() {
    print_layer(&layer);
  }
}
