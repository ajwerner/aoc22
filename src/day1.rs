use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn parse(input_str: &str) -> Vec<Vec<u64>> {
    let mut res: Vec<Vec<u64>> = Vec::new();
    let mut curb: Option<Vec<u64>> = Some(Vec::new());
    for x in input_str.split("\n") {
        if let Some(ref mut cur) = curb {
            match x {
                "" => {
                    res.push(curb.take().unwrap());
                    curb = Some(Vec::new());
                }
                _ => cur.push(x.parse::<u64>().unwrap()),
            }
        }
    }
    return res;
}

pub fn part1(input: Vec<Vec<u64>>) -> u64 {
    input
        .into_iter()
        .map(|l| l.into_iter().sum())
        .max()
        .unwrap()
}

pub fn part2(input: Vec<Vec<u64>>) -> u64 {
    let mut h2: BinaryHeap<Reverse<u64>> = BinaryHeap::new();
    input
        .into_iter()
        .map(|l| l.into_iter().sum())
        .fold(&mut h2, |h, s| {
            h.push(Reverse(s));
            if h.len() > 3 {
                h.pop();
            }
            return h;
        })
        .to_owned()
        .into_iter()
        .map(|x| x.0)
        .sum()
}
