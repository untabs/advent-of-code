// use indicatif::ProgressIterator;
use std::fs;
use std::ops::Range;
use std::time::Instant;

use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending, space0, space1},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug)]
struct MyRange {
    start: i64,
    end: i64,
}

#[derive(Debug)]
struct Instruction {
    range: MyRange,
    addition_value: i64,
}

fn line(input: &str) -> IResult<&str, Instruction> {
    let (input, (destination, source, num)) = tuple((
        complete::i64.preceded_by(space0),
        complete::i64.preceded_by(space0),
        complete::i64.preceded_by(space0),
    ))(input)?;

    Ok((
        input,
        Instruction {
            range: MyRange {
                start: source,
                end: (source + num) - 1,
            },
            addition_value: destination - source,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, (Vec<i64>, Vec<Vec<Instruction>>)> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(space1, complete::i64))
        .parse(input)?;

    let (input, _) = take_until("map:").parse(input)?;
    let (input, maps) = many1(
        take_until("map:")
            .precedes(tag("map:"))
            .precedes(many1(line_ending.precedes(line))),
    )
    .parse(input)?;
    Ok((input, (seeds, maps)))
}

fn main() {
    let file_path = "input1.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let current = Instant::now();
    println!("{}", part1(&input));
    let duration = current.elapsed();
    println!("duration: {:?}", duration);
}

fn part1(input: &str) -> String {
    let (_, (seeds_and_range, maps)) = parse(input).expect("ERM PARSER DOESNT WORK??");
    let mut seeds: Vec<Range<i64>> = vec![];
    for i in (0..seeds_and_range.len()).step_by(2) {
        seeds.push(seeds_and_range[i]..(seeds_and_range[i] + seeds_and_range[i + 1]));
    }

    let answer = seeds
        .iter()
        .map(|s_range| {
            println!("{:?}", s_range);
            s_range
                .clone()
                .into_par_iter()
                .map(|value| {
                    let mut out: i64 = value;
                    maps.iter().for_each(|instruction_guide| {
                        for instruction in instruction_guide {
                            if out >= instruction.range.start && out <= instruction.range.end {
                                out += instruction.addition_value;
                                break;
                            }
                        }
                    });
                    out
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    answer.to_string()
}

#[cfg(test)]

mod test {

    #[test]

    fn given_io() {
        use super::*;

        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15
12634632
fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = part1(input);
        assert_eq!("46", result)
    }
}
