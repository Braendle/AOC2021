use anyhow::Context;
use itertools::Itertools;
use std::{fs, iter::successors, str::FromStr};

fn main() {
    let raw = fs::read_to_string("assets/day05.txt").expect("could not open file!");
    let lines: Vec<Line> = raw.lines().map(|l| l.parse().unwrap()).collect();
    let lines_filtered: Vec<_> = lines
        .iter()
        .filter(|l| l.is_horz_or_vert())
        .cloned()
        .collect();

    dbg!(count_overlaps(&lines_filtered));
    dbg!(count_overlaps(&lines));
}

fn count_overlaps(lines: &[Line]) -> usize {
    lines
        .iter()
        .flat_map(|l| l.covered_points())
        .counts()
        .iter()
        .filter(|e| *e.1 >= 2)
        .count()
}

#[derive(Debug, Clone)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl FromStr for Line {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x1, y1, x2, y2) = s
            .split(" -> ")
            .flat_map(|p| p.split(','))
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .context("unable to unpack tuple")?;
        Ok(Self { x1, y1, x2, y2 })
    }
}

fn dir_range(start: usize, end: usize) -> impl Iterator<Item = usize> {
    let dx = end.cmp(&start) as isize;
    successors(Some(start), move |&x| {
        (x != end).then(|| (x as isize + dx) as usize)
    })
}

impl Line {
    fn is_horz_or_vert(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }
    fn covered_points(&self) -> Vec<(usize, usize)> {
        if self.x1 == self.x2 {
            dir_range(self.y1, self.y2).map(|y| (self.x1, y)).collect()
        } else if self.y1 == self.y2 {
            dir_range(self.x1, self.x2).map(|x| (x, self.y1)).collect()
        } else {
            dir_range(self.x1, self.x2)
                .zip(dir_range(self.y1, self.y2))
                .collect()
        }
    }
}
