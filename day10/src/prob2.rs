use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

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

#[derive(PartialEq, Eq, Debug, Clone)]
struct Point {
  x: i32,
  y: i32,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Alphas {
  alpha: i32,
  beta: i32,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Frac {
  alpha: i32,
  beta: i32,
}

#[derive(Debug)]
struct DataPart {
  straight: Vec<Alphas>,
  // Store just the divisor
  diags: HashMap<Frac, Vec<i32>>,
}

impl DataPart {
  fn print(&self) {
    for s in self.straight.iter() {
      println!("{:?}", s);
    }
    for d in self.diags.iter() {
      println!("{:?}: {:?}", d.0, d.1);
    }
    println!("---");
  }

  /**
   * Returns None if didn't catch a thing
   */
  fn remove_elems(&mut self, count: &mut i32) -> Option<Alphas> {
    // Elements in straight are sorted increasingly
    if self.straight.len() > 0 {
      let elem = self.straight.pop().unwrap();
      *count -= 1;
      if *count == 0 {
        return Some(elem);
      }
    }

    let keys = {
      let mut keys: Vec<Frac> = self.diags.keys().cloned().collect();
      keys.sort_by(|a, b| {
        let fa = (a.alpha as f64) / (a.beta as f64);
        let fb = (b.alpha as f64) / (b.beta as f64);

        fa.partial_cmp(&fb).unwrap()
      });
      keys
    };

    for key in keys.iter() {
      let vec = self.diags.get_mut(&key).unwrap();
      let elem_gcd = vec.pop().unwrap();
      *count -= 1;
      if *count == 0 {
        let elem = Alphas {
          alpha: key.alpha * elem_gcd,
          beta: key.beta * elem_gcd,
        };
        return Some(elem);
      }
    }

    // returns how many elements it has removed
    // also returns if count is 0
    None
  }
}

fn arr_to_par(max_alpha: i32, max_beta: i32, alpha_contains: impl Fn(Alphas) -> bool) -> DataPart {
  // Populate the top-right part

  // Don't care about overflow
  let mut straight: Vec<Alphas> = vec![];
  for beta in 1..max_beta {
    // if within bounds
    if alpha_contains(Alphas { alpha: 0, beta }) {
      straight.push(Alphas { alpha: 0, beta });
    }
  }
  // So that elements can be popped easily
  straight.reverse();

  let mut diags: HashMap<Frac, Vec<i32>> = HashMap::new();
  for beta in 1..max_beta {
    for alpha in 1..max_alpha {
      if !alpha_contains(Alphas { alpha, beta }) {
        continue;
      }
      let greatest = num::integer::gcd(alpha, beta);

      let frac = Frac {
        alpha: alpha / greatest,
        beta: beta / greatest,
      };

      match diags.get_mut(&frac) {
        Some(vecs) => {
          vecs.push(greatest);
        }
        None => {
          diags.insert(frac, vec![greatest]);
        }
      }
    }
  }

  // Sort diags so that can be easily popped
  for frac in diags.iter_mut() {
    let f = frac.1;
    f.sort();
    f.reverse();
  }

  DataPart { straight, diags }
}

fn arr_to_data(arr: &Vec<Vec<char>>, start: Point, width: i32, height: i32) -> Point {
  let point_is_good =
    |p: &Point| within_bounds(p.x, p.y, width, height) && arr[p.y as usize][p.x as usize] == '#';

  let tr_alpha_point = |albeta: Alphas| {
    let x = albeta.alpha + start.x;
    let y = start.y - albeta.beta;
    Point { x, y }
  };

  let rd_alpha_point = |albeta: Alphas| {
    let x = start.x + albeta.beta;
    let y = start.y + albeta.alpha;
    Point { x, y }
  };

  let dl_alpha_point = |albeta: Alphas| {
    let x = start.x - albeta.alpha;
    let y = start.y + albeta.beta;
    Point { x, y }
  };

  let lu_alpha_point = |albeta: Alphas| {
    let x = start.x - albeta.beta;
    let y = start.y - albeta.alpha;
    Point { x, y }
  };

  let mut data_top_right = {
    let top_right_contains = |albeta: Alphas| {
      let p = tr_alpha_point(albeta);
      point_is_good(&p)
    };
    arr_to_par(width, height, top_right_contains)
  };

  let mut data_right_down = {
    let right_down_contains = |albeta: Alphas| {
      let p = rd_alpha_point(albeta);
      point_is_good(&p)
    };
    arr_to_par(width, height, right_down_contains)
  };

  let mut data_down_left = {
    let down_left_contains = |albeta: Alphas| {
      let p = dl_alpha_point(albeta);
      point_is_good(&p)
    };
    arr_to_par(width, height, down_left_contains)
  };

  let mut data_left_up = {
    let left_up_contains = |albeta: Alphas| {
      let p = lu_alpha_point(albeta);
      point_is_good(&p)
    };
    arr_to_par(width, height, left_up_contains)
  };

  data_top_right.print();
  data_right_down.print();
  data_down_left.print();
  data_left_up.print();

  loop {
    let mut count = 200;

    if let Some(val) = data_top_right.remove_elems(&mut count) {
      return tr_alpha_point(val);
    }

    if let Some(val) = data_right_down.remove_elems(&mut count) {
      return rd_alpha_point(val);
    }

    if let Some(val) = data_down_left.remove_elems(&mut count) {
      return dl_alpha_point(val);
    }

    if let Some(val) = data_left_up.remove_elems(&mut count) {
      return lu_alpha_point(val);
    }
  }
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

  let p = arr_to_data(&arr, Point { x: 25, y: 31 }, width as i32, height as i32);

  println!("point: {:?}", p);
}
