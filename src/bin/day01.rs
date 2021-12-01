use std::fs;

fn main() {
    let raw = fs::read_to_string("assets/day01.txt").expect("could not open file!");
    let data: Vec<i32> = raw.lines().map(|x| x.parse().unwrap()).collect();

    let a = data.windows(2).filter(|x| x[0] < x[1]).count();
    dbg!(a);

    let sums: Vec<i32> = data.windows(3).map(|x| x.iter().sum()).collect();
    let b = sums.windows(2).filter(|x| x[0] < x[1]).count();
    dbg!(b);
    // or use the fact that (x1 + x2 + x3) - (x0 + x1 + x2) = x3 - x0
    let b = data.windows(4).filter(|x| x[0] < x[3]).count();
    dbg!(b);
}
