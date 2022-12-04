use std::ops::{Add, Rem, Sub};

#[derive(Debug)]
pub struct Round1(Shape, Shape);

pub struct Round2(Shape, Outcome);

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

pub fn make_round1(v: (char, char)) -> Round1 {
    Round1(Shape::from_char(v.0), Shape::from_char(v.1))
}

pub fn make_round2(v: (char, char)) -> Round2 {
    Round2(Shape::from_char(v.0), Outcome::from_char(v.1))
}

impl Outcome {
    fn from_char(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => {
                panic!("unknown char {}", c);
            }
        }
    }

    fn val(self) -> i32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 1,
            Outcome::Win => 2,
        }
    }

    fn my_hand(self, their_hand: Shape) -> Shape {
        Shape::from_val(modulus(their_hand.val() - 2 + self.val(), 3) + 1)
    }
}

impl Shape {
    fn from_char(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => {
                panic!("unknown char {}", c);
            }
        }
    }
    fn val(self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
    fn from_val(v: i32) -> Self {
        match v {
            1 => Self::Rock,
            2 => Self::Paper,
            3 => Self::Scissors,
            _ => {
                panic!("unknown val {}", v);
            }
        }
    }
}

impl Round1 {
    fn val(self) -> i32 {
        self.1.val() + modulus(self.1.val() - self.0.val() + 1, 3) * 3
    }
}

impl Round2 {
    fn to_round1(self) -> Round1 {
        Round1(self.0, self.1.my_hand(self.0))
    }
}

pub fn parse(input: &str) -> Vec<(char, char)> {
    input
        .split("\n")
        .into_iter()
        .map(|s| {
            let x: Vec<char> = s
                .chars()
                .into_iter()
                .filter(|c| !c.is_whitespace())
                .collect();
            (x[0], x[1])
        })
        .collect()
}

fn modulus<T: Add<Output = T> + Sub<Output = T> + Rem<Output = T> + Ord + Default + Copy>(
    a: T,
    b: T,
) -> T {
    let v = a % b;
    if v < T::default() {
        v + b
    } else {
        v
    }
}

pub fn part1(input: Vec<(char, char)>) -> i32 {
    input.into_iter().map(make_round1).map(Round1::val).sum()
}

pub fn part2(input: Vec<(char, char)>) -> i32 {
    input
        .into_iter()
        .map(make_round2)
        .map(Round2::to_round1)
        .map(Round1::val)
        .sum()
}
