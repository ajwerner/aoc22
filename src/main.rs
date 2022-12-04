use clap::{Parser, Subcommand};
mod day1;
mod day2;

#[derive(Debug, Parser)]
#[command(name = "aoc22")]
#[command(about = "Advent Of Code 2022", long_about = None)]
struct Cli {
    #[command(subcommand)]
    day: Days,
}

#[derive(Debug, Subcommand)]
enum Days {
    Day1 {
        #[command(subcommand)]
        part: Parts,
    },
    Day2 {
        #[command(subcommand)]
        part: Parts,
    },
}

#[derive(Debug, Subcommand)]
enum Parts {
    Part1 {},
    Part2 {},
}

fn main() {
    let args = Cli::parse();

    match args.day {
        Days::Day1 { part } => {
            let input = day1::parse(include_str!("day1.txt"));
            match part {
                Parts::Part1 {} => {
                    println!("{}", day1::part1(input));
                }
                Parts::Part2 {} => {
                    println!("{}", day1::part2(input))
                }
            }
        }
        Days::Day2 { part } => {
            let input = day2::parse(include_str!("day2.txt"));
            match part {
                Parts::Part1 {} => {
                    println!("{}", day2::part1(input));
                }
                Parts::Part2 {} => {
                    println!("{}", day2::part2(input));
                }
            }
        }
    }
}
