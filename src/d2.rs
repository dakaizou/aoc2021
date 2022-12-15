use crate::util::read_lines;

#[derive(Debug)]
struct Submarine {
    x: usize,
    y: isize,
}

impl Submarine {
    fn new() -> Self {
        Submarine { x: 0, y: 0 }
    }

    fn mv(&mut self, cmd: Command) {
        match cmd {
            Command::Up(n) => self.y -= n as isize,
            Command::Down(n) => self.y += n as isize,
            Command::Forward(n) => self.x += n,
        }
    }

    fn mult(&self) -> isize {
        self.x as isize * self.y
    }
}

struct Submarine2 {
    x: usize,
    y: isize,
    aim: isize,
}

impl Submarine2 {
    fn new() -> Self {
        Submarine2 { x: 0, y: 0, aim: 0}
    }

    fn mv(&mut self, cmd: Command) {
        match cmd {
            Command::Up(n) => self.aim -= n as isize,
            Command::Down(n) => self.aim += n as isize,
            Command::Forward(n) => {
                self.x += n;
                self.y += self.aim * n as isize;
            },
        }
    }

    fn mult(&self) -> isize {
        self.x as isize * self.y
    }
}

#[derive(Debug)]
enum Command {
    Up(usize),
    Down(usize),
    Forward(usize),
}

fn parse_line(line: &str) -> Command {
    let mut line = line.split_whitespace();
    let command_str = line.next().unwrap();
    let num = line.next().unwrap().parse::<usize>().unwrap();
    match command_str {
        "up" => Command::Up(num),
        "down" => Command::Down(num),
        "forward" => Command::Forward(num),
        _ => panic!("no such command"),
    }
}

#[allow(dead_code)]
fn p1() {
    let lines = read_lines("input/d2");
    let mut sub = Submarine::new();
    for line in lines {
        sub.mv(parse_line(&line.unwrap()));
    }
    println!("{}", sub.mult());
}

#[allow(dead_code)]
fn p2() {
    let lines = read_lines("input/d2");
    let mut sub = Submarine2::new();
    for line in lines {
        sub.mv(parse_line(&line.unwrap()));
    }
    println!("{}", sub.mult());
}

pub fn main() {
    p2();
}
