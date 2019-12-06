use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug)]
enum Path {
    Right(i32),
    Left(i32),
    Up(i32),
    Down(i32),
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Point {
        Point { x: 0, y: 0 }
    }
}

fn str_to_path(s: &str) -> Path {
    match s.chars().next().unwrap() {
        'R' => Path::Right(s[1..].parse().unwrap()),
        'L' => Path::Left(s[1..].parse().unwrap()),
        'U' => Path::Up(s[1..].parse().unwrap()),
        'D' => Path::Down(s[1..].parse().unwrap()),
        _ => panic!("Letter not understood"),
    }
}

fn set_from_string(contents: &str) -> HashSet<Point> {
    let paths: Vec<Path> = contents.split(",").map(|s| str_to_path(s)).collect();
    let mut points = HashSet::new();

    let mut currentPoint = Point::new();

    for path in paths.iter() {
        match path {
            Path::Right(num) => {
                for i in (currentPoint.x + 1)..=(currentPoint.x + num) {
                    points.insert(Point {
                        x: i,
                        y: currentPoint.y,
                    });
                }

                currentPoint.x += num;
            }
            Path::Left(num) => {
                for i in ((currentPoint.x - num)..(currentPoint.x)).rev() {
                    points.insert(Point {
                        x: i,
                        y: currentPoint.y,
                    });
                }

                currentPoint.x -= num;
            }
            Path::Up(num) => {
                for i in (currentPoint.y + 1)..=(currentPoint.y + num) {
                    points.insert(Point {
                        x: currentPoint.x,
                        y: i,
                    });
                }

                currentPoint.y += num;
            }
            Path::Down(num) => {
                for i in ((currentPoint.y - num)..(currentPoint.y)).rev() {
                    points.insert(Point {
                        x: currentPoint.x,
                        y: i,
                    });
                }

                currentPoint.y -= num;
            }
        }
    }

    points
}

fn insert_smallest(map: &mut HashMap<Point, i32>, point: Point, dist: i32) {
    let f = map.get(&point);

    match f {
        Some(existing) => {
            // if &dist < existing {
            //     map.insert(point, dist);
            // }
        }
        None => {
            map.insert(point, dist);
        }
    }
}

fn map_from_string(contents: &str) -> HashMap<Point, i32> {
    let paths: Vec<Path> = contents.split(",").map(|s| str_to_path(s)).collect();
    let mut points = HashMap::new();

    let mut currentPoint = Point::new();
    let mut currentDistance = 0;

    for path in paths.iter() {
        match path {
            Path::Right(num) => {
                for i in 1..=(0 + num) {
                    insert_smallest(
                        &mut points,
                        Point {
                            x: currentPoint.x + i,
                            y: currentPoint.y,
                        },
                        currentDistance + i,
                    )
                }

                currentDistance += num;
                currentPoint.x += num;
            }
            Path::Left(num) => {
                for i in 1..(0 + num) {
                    insert_smallest(
                        &mut points,
                        Point {
                            x: currentPoint.x - i,
                            y: currentPoint.y,
                        },
                        currentDistance + i,
                    );
                }

                currentDistance += num;
                currentPoint.x -= num;
            }
            Path::Up(num) => {
                for i in 1..=(0 + num) {
                    insert_smallest(
                        &mut points,
                        Point {
                            x: currentPoint.x,
                            y: currentPoint.y + i,
                        },
                        currentDistance + i,
                    );
                }

                currentDistance += num;
                currentPoint.y += num;
            }
            Path::Down(num) => {
                for i in 1..=(0 + num) {
                    insert_smallest(
                        &mut points,
                        Point {
                            x: currentPoint.x,
                            y: currentPoint.y - i,
                        },
                        currentDistance + i,
                    );
                }

                currentDistance += num;
                currentPoint.y -= num;
            }
        }
    }

    points
}

fn main1() {
    let full_file = fs::read_to_string("input.txt").expect("File couldn't be read");
    let mut lines = full_file.lines();
    let line1 = lines.next().unwrap();
    let line2 = lines.next().unwrap();

    let points1 = set_from_string(line1);
    let points2 = set_from_string(line2);

    let intersection_points: HashSet<&Point> = points1.intersection(&points2).collect();

    let fuck: Vec<i32> = intersection_points
        .into_iter()
        .map(|p| p.x.abs() + p.y.abs())
        .collect();

    let min = fuck.iter().min().unwrap();

    println!("{}", line1);
    println!("{}", line2);

    for seg in fuck.iter() {
        println!("{:?}", seg);
    }

    println!("min: {}", min);
}

fn print_points(points1: &HashMap<Point, i32>, points2: &HashMap<Point, i32>) {
    for y in (-5..30).rev() {
        for x in -5..30 {
            let c1 = points1.contains_key(&Point { x: x, y: y });
            let c2 = points2.contains_key(&Point { x: x, y: y });

            if c1 && c2 {
                let g1 = points1.get(&Point { x: x, y: y }).unwrap();
                let g2 = points2.get(&Point { x: x, y: y }).unwrap();

                print!(" (@{:02},{:02})", g1, g2);
            } else if c1 {
                let g1 = points1.get(&Point { x: x, y: y }).unwrap();

                print!(" (x{:02},  )", g1);
            } else if c2 {
                let g2 = points2.get(&Point { x: x, y: y }).unwrap();

                print!(" (&  ,{:02})", g2);
            } else {
                print!("         ")
            }
        }

        println!("|");
    }
}

fn main() {
    let full_file = fs::read_to_string("input.txt").expect("File couldn't be read");
    let mut lines = full_file.lines();
    let line1 = lines.next().unwrap();
    let line2 = lines.next().unwrap();

    let points1 = map_from_string(line1);
    let points2 = map_from_string(line2);

    print_points(&points1, &points2);

    {
        let mut minimalDist = std::i32::MAX;

        // Find intersections manually
        for (point, distance) in &points1 {
            let inPoints2 = points2.get(point);

            match inPoints2 {
                Some(dist) => {
                    println!(
                        "p1: {:?}, {} {}, add: {}",
                        point,
                        distance,
                        dist,
                        distance + dist
                    );

                    if dist + distance < minimalDist {
                        // println!("p1: {:?}, {} {}", point, distance, dist);
                        minimalDist = dist + distance;
                    };
                    minimalDist = cmp::min(minimalDist, dist + distance);
                }
                None => {}
            }
        }

        println!("minimal {}", minimalDist);
    }
}
