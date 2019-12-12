use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Point {
  x: i32,
  y: i32,
}

pub fn spiral(level: i32) -> Vec<(i32, i32)> {
  let mut spiral: Vec<(i32, i32)> = vec![];

  if level < 0 {
    panic!("level cannot be negative");
  }
  let i = level;
  // Right side starting from the upper corner
  for y in -i..i {
    spiral.push((i, y));
  }

  // bottom
  let mut x = i;
  while x > -i {
    spiral.push((x, i));

    x -= 1;
  }

  // left
  let mut y = i;
  while y > -i {
    spiral.push((-i, y));

    y -= 1;
  }

  for x in -i..i {
    spiral.push((x, -i));
  }

  spiral
}

fn create_visible_arr(height: usize, width: usize) -> Vec<Vec<bool>> {
  let mut visible: Vec<Vec<bool>> = vec![];

  for y in 0..height {
    let mut line: Vec<bool> = vec![];

    for x in 0..width {
      line.push(true);
    }

    visible.push(line)
  }

  visible
}

fn print_visible(visible: &Vec<Vec<bool>>) {
  for line in visible.iter() {
    for cell in line.iter() {
      print!("{}", if *cell { "." } else { "X" });
    }
    println!();
  }
  println!("----------")
}

fn within_bounds(x: i32, y: i32, width: i32, height: i32) -> bool {
  let val = x >= 0 && x < width && y >= 0 && y < height;
  // println!(
  //   "checking bounds, x:{}, y:{}, width:{}, height:{}, val:{}",
  //   x, y, width, height, val
  // );
  val
}

pub fn main() {
  // Reading part
  let contents = fs::read_to_string("input.txt").expect("File couldn't be read");
  let lines = contents.lines();

  let mut height: usize = 0;
  let mut width: usize = 0;
  let mut arr: Vec<Vec<char>> = vec![];
  for line in lines {
    let chars: Vec<char> = line.chars().collect();
    width = chars.len();
    arr.push(chars);
    height += 1;
  }

  println!("height:{}, width:{}", height, width);

  let mut max_count = -1;
  let mut max_count_at = None;

  // even though max_level should be sqrt(2) * something
  let max_level = height + width;
  for y in 0..height {
    for x in 0..width {
      if arr[y][x] == '#' {
        //
        let mut visible = create_visible_arr(height, width);

        // Don't check the current location
        visible[y][x] = false;
        let mut count = 0;

        for level in 1..max_level {
          let possibilities = spiral(level as i32);

          for poss in possibilities.iter() {
            let px = (x as i32) + poss.0;
            let py = (y as i32) + poss.1;
            // println!("px:{}, py:{}", px, py);
            // println!("{:?}\n", poss);

            // Check bounds
            if !within_bounds(px as i32, py as i32, width as i32, height as i32) {
              continue;
            }
            // println!("px:{}, py:{}", px, py);
            let px: usize = px as usize;
            let py: usize = py as usize;
            if arr[py][px] == '#' && visible[py][px] {
              // println!("visible asteroid, {}, {}", px, py);

              count += 1;
              let greatest = num::integer::gcd(poss.0, poss.1);
              let gpx = (poss.0 / greatest) as i32;
              let gpy = (poss.1 / greatest) as i32;

              let mut tx = x as i32;
              let mut ty = y as i32;
              while within_bounds(tx, ty, width as i32, height as i32) {
                visible[ty as usize][tx as usize] = false;
                tx += gpx;
                ty += gpy;
              }

              // print_visible(&visible);
            }
          }
        }

        println!("at x:{}, y:{}, count:{}", x, y, count);

        if count > max_count {
          max_count_at = Some(Point {
            x: x as i32,
            y: y as i32,
          });
          max_count = count;
        }
      }
    }
  }

  println!("max count: {}, at: {:?}", max_count, max_count_at);
}
