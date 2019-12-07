use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;

fn find_orbits(current: &str, lines: &Vec<&str>, depth: i32) -> i32 {
    let mut count = depth;

    // This seems to be a bottleneck
    for line in lines.iter() {
        let vals: Vec<&str> = line.split(")").collect();
        let parent = vals[0];
        let child = vals[1];

        if parent != current {
            continue;
        }

        let child_orbits = find_orbits(child, &lines, depth + 1);

        count += child_orbits;
    }
    // println!("finished {}, {}", current, count);

    count
}

fn main() {
    // Reading part
    let contents = fs::read_to_string("input.txt").expect("File couldn't be read");
    let lines: Vec<&str> = contents.lines().collect();

    // Find all planets that have children
    let mut all_objects: HashSet<&str> = HashSet::new();

    let mut just_children: HashSet<&str> = HashSet::new();

    for line in lines.iter() {
        let vals: Vec<&str> = line.split(")").collect();
        let parent = vals[0];
        let child = vals[1];

        // Some insertions may be redundant
        all_objects.insert(parent);
        all_objects.insert(child);

        just_children.insert(child);
    }

    // Find the ones that don't have parents
    let starting_objects: HashSet<_> = all_objects.difference(&just_children).collect();

    for o in starting_objects.iter() {
        println!("STARTING OBJECT: {}", o);
        let count = find_orbits(o, &lines, 0);

        println!("count: {}", count);
    }
}
