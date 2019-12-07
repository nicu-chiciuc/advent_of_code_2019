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

fn print_tree(tree: &HashMap<&str, Vec<&str>>) {
    for (parent, child) in tree.iter() {
        print!("parent {} has the children: ", parent);

        for i in child.iter() {
            print!("{}, ", i);
        }
        println!();
    }
}

fn print_vec(v: &Vec<String>) {
    for i in v.iter() {
        print!("{}, ", i);
    }
    println!();
}

fn path_from_to<'a>(
    from: String,
    to: &str,
    tree: &HashMap<&str, Vec<String>>,
    acc: &mut Vec<String>,
) -> bool {
    if from == to {
        acc.push(from);
        return true;
    }

    let children = tree.get(&from[..]);
    if children == None {
        return false;
    }

    for child in children.unwrap().iter() {
        acc.push(from.clone());
        let path = path_from_to(child.clone(), &to, &tree, acc);

        if path {
            return true;
        };

        acc.pop();
    }

    false
}

pub fn main() {
    // Reading part
    let contents = fs::read_to_string("input.txt").expect("File couldn't be read");
    let lines: Vec<&str> = contents.lines().collect();

    // Find all planets that have children
    let mut all_objects: HashSet<&str> = HashSet::new();
    let mut just_children: HashSet<&str> = HashSet::new();

    let mut tree: HashMap<&str, Vec<String>> = HashMap::new();

    for line in lines.iter() {
        let vals: Vec<&str> = line.split(")").collect();
        let parent = vals[0];
        let child = vals[1];

        match tree.get_mut(parent) {
            Some(children) => {
                children.push(String::from(child));
            }
            None => {
                tree.insert(parent, vec![String::from(child)]);
            }
        };

        // Some insertions may be redundant
        all_objects.insert(parent);
        all_objects.insert(child);

        just_children.insert(child);
    }

    // Find the ones that don't have parents
    let starting_objects: HashSet<_> = all_objects.difference(&just_children).collect();

    for o in starting_objects.iter() {
        let mut path_to_you: Vec<String> = vec![];
        path_from_to(String::from(**o), "YOU", &tree, &mut path_to_you);

        let mut path_to_san: Vec<String> = vec![];
        path_from_to(String::from(**o), "SAN", &tree, &mut path_to_san);

        print_vec(&path_to_you);
        print_vec(&path_to_san);

        let mut i = 0;

        loop {
            if path_to_san[i] != path_to_you[i] {
                let from_you_to_mid = path_to_you.len() - i - 1;
                let from_san_to_mid = path_to_san.len() - i - 1;
                let sum = from_you_to_mid + from_san_to_mid;

                println!("{}, {}, {}", from_you_to_mid, from_san_to_mid, sum);
                break;
            }

            i += 1;
        }
    }
}
