use crate::util::read_lines;
use crate::util::Windowing;

#[allow(dead_code)]
fn p1() {
    let mut lines = read_lines("input/d1");
    let mut prev = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    let mut inc_count = 0;

    for line in lines {
        let cur = line.unwrap().parse::<i32>().unwrap();
        if cur > prev {
            inc_count += 1;
        }
        prev = cur;
    }

    println!("{inc_count}");
}

#[allow(dead_code)]
fn p2() {
    let mut lines = read_lines("input/d1").window(3);
    let mut prev = lines
        .next()
        .unwrap()
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .sum::<i32>();
    let mut inc_count = 0;

    for window in lines {
        let cur = window.iter().map(|s| s.parse::<i32>().unwrap()).sum();
        if cur > prev {
            inc_count += 1;
        }
        prev = cur;
    }

    println!("{inc_count}");
}

#[allow(dead_code)]
pub fn main() {
    p2();
}
