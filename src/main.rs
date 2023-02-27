use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io::{self, BufRead};


#[derive(Clone, Copy)]
enum RopeMoveDirection {
    Up, Down, Left, Right
}


struct RopeMove {
    direction: RopeMoveDirection,
    distance: usize
}

struct RopeKnot {
    x: isize,  // position
    y: isize,
    next: Option<Box<RopeKnot>>
}

impl RopeKnot {

    fn new(length: usize) -> Self {
        let mut head: RopeKnot = RopeKnot {
            x: 0,
            y: 0,
            next: None
        };
        for _ in 0..length {
            head = RopeKnot {
                x: 0,
                y: 0,
                next: Some(Box::new(head))
            }
        }
        head
    }

    fn propagate(&mut self) {
        if let Some(next_knot) = &mut self.next {
            next_knot.follow(self.x, self.y);
        }
    }

    /// move the knot directly
    fn apply_move(&mut self, rope_move: RopeMove) {
        for _ in 0..rope_move.distance {
            match rope_move.direction {
                RopeMoveDirection::Down => self.y -= 1,
                RopeMoveDirection::Up => self.y += 1,
                RopeMoveDirection::Left => self.x -= 1,
                RopeMoveDirection::Right => self.x += 1
            }
            self.propagate();
        }
    }

    /// update the position of the knot to follow towards a new set of coordinates
    fn follow(&mut self, x: isize, y: isize) {
        if isize::abs(self.x - x) > 1 || isize::abs(self.y - y) > 1 {
            match self.x - x {
                std::isize::MIN..=-1 => self.x += 1,
                1..=std::isize::MAX => self.x -= 1,
                _ => ()
            }
            match self.y - y {
                std::isize::MIN..=-1 => self.y += 1,
                1..=std::isize::MAX => self.y -= 1,
                _ => ()
            }
            self.propagate();
        }

    }

    fn list_positions(&self, skip: isize) -> Vec<(isize, isize)> {
        let mut starter = match &self.next {
            Some(next_knot) => next_knot.list_positions(skip - 1),
            None => vec![]
        };
            
        if skip <= 0 {
            starter.push((self.x, self.y));
        }

        starter
    }

    fn show(&self) {
        let positions = self.list_positions(0);
        let x_min: isize = positions.iter().map(|c| c.0).min().unwrap();
        let x_max: isize = positions.iter().map(|c| c.0).max().unwrap();
        let y_min: isize = positions.iter().map(|c| c.1).min().unwrap();
        let y_max: isize = positions.iter().map(|c| c.1).max().unwrap();

        println!("_({}, {})", x_min - 1, y_min - 1);
        println!("|");

        for i in x_min-1..=x_max+1 {
            for j in y_min-1..=y_max+1 {
                if positions.contains(&(i, j)) {
                    print!("R");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}


fn main() {
    let path = env::args().nth(1).expect("No file path provided!");

    let data = io::BufReader::new(
        fs::File::open(path).expect("Could not open file!")
        ).lines()
        .map(|line| {
            let text = line.unwrap();
            RopeMove { 
                direction: match text.chars().nth(0).expect("Bad move") {
                    'R' => RopeMoveDirection::Right,
                    'L' => RopeMoveDirection::Left,
                    'U' => RopeMoveDirection::Up,
                    'D' => RopeMoveDirection::Down,
                    _ => panic!("Invalid move!")
                }, 
                distance: text.split_at(2).1.parse::<usize>().expect("invalid distance!")
            }
        }).flat_map(|rope_move| {
            let mut result: Vec<RopeMove> = vec![];
            for _ in 0..rope_move.distance {
                result.push(RopeMove {
                    direction: rope_move.direction,
                    distance: 1
                });
            }
            result.into_iter()
        });

    let mut rope = RopeKnot::new(9);
    let mut counter: usize = 0;
    let mut visited: BTreeSet<(isize, isize)> = BTreeSet::new();
    for rope_move in data.into_iter() {
        rope.apply_move(rope_move);
        visited.extend(rope.list_positions(9).into_iter());
        counter += 1;
        rope.show();
    }

    println!("{} moves applied", counter);
    println!("{} positions visited by the tail.", visited.len());

}
