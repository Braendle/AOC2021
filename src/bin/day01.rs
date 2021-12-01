use std::fs;

fn main() {
    let raw = fs::read_to_string("assets/day01.txt").expect("could not open file!");
    let data: Vec<i32> = raw.lines().map(|x| x.parse().unwrap()).collect();

    let a = data.windows(2).map(|x| x[1] - x[0]).filter(|&x| x > 0).count();
    dbg!(a);

    let sums: Vec<i32> = data.windows(3).map(|x| x.iter().sum()).collect();
    let b = sums.windows(2).map(|x| x[1] - x[0]).filter(|&x| x > 0).count();
    dbg!(b);
}
