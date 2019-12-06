use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn is_good_num(numi: i32) -> bool {
    let mut v: Vec<i32> = vec![];

    let mut num = numi;

    for i in 0..6 {
        let b = num % 10;
        v.push(b);
        num = num / 10;
    }

    v.reverse();
    let mut had_pair = false;

    let mut i = 0;

    while i < 6 {
        let mut j = i + 1;
        let mut groupCount = 1;

        while j < 6 && v[i] == v[j] {
            groupCount += 1;
            j += 1;
        }

        // println!("group {} of {}", groupCount, v[i]);

        if groupCount == 2 {
            had_pair = true;
        }
        if j < 6 && v[i] > v[j] {
            return false;
        }

        i = j;
    }

    return had_pair;
}

fn main() {
    let full_file = fs::read_to_string("input.txt").expect("File couldn't be read");
    let nums: Vec<i32> = full_file.split("-").map(|d| d.parse().unwrap()).collect();
    let min = nums[0];
    let max = nums[1];

    let mut count = 0;

    // is_good_num(111223);
    for i in min..=max {
        if is_good_num(i) {
            count += 1;
        }
    }
    println!("{}", count);
}
