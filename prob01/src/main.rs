use std::env;
use std::fs;

fn calc(num: i128) -> i128 {
    let f: i128 = num / 3;
    f - 2
}

fn calc_rec(num: i128, acc: i128) -> i128 {
    let fuelNow = calc(num);

    if fuelNow > 0 {
        calc_rec(fuelNow, acc + fuelNow)
    } else {
        acc
    }
}

fn prob01(nums: &Vec<i128>) -> i128 {
    let sum: i128 = nums.iter().map(|num| calc(*num)).sum();

    sum
}

fn prob02(nums: &Vec<i128>) -> i128 {
    let sum: i128 = nums.iter().map(|num| calc_rec(*num, 0)).sum();

    sum
}

fn main() {
    // Reading part
    let contents = fs::read_to_string("prob01_exer.txt").expect("File couldn't be read");
    let lines: Vec<i128> = contents.lines().map(|line| line.parse().unwrap()).collect();

    // apply the function to all elements
    let sum = prob02(&lines);

    // let b = calc_rec(100756, 0);

    println!("sum: {}", sum);
}
