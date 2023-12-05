use core::num;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    num::ParseIntError,
    vec,
};

const seed_to_soil: &str = "seed-to-soil";
const soil_to_fertilizer: &str = "soil-to-fertilizer";
const fertilizer_to_water: &str = "fertilizer-to-water";
const water_to_light: &str = "water-to-light";
const light_to_temperature: &str = "light-to-temperature";
const temperature_to_humidity: &str = "temperature-to-humidity";
const humidity_to_location: &str = "humidity-to-location";

struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Vec<Line>,
    soil_to_fertilizer: Vec<Line>,
    fertilizer_to_water: Vec<Line>,
    water_to_light: Vec<Line>,
    light_to_temperature: Vec<Line>,
    temperature_to_humidity: Vec<Line>,
    humidity_to_location: Vec<Line>,
}

impl Almanac {
    fn set_seed_numbers(&mut self, data: &str) {
        let numbers = get_numbers(data);
        for i in numbers.chunks(2).into_iter() {
            let v = i[0];
            let c = i[1];
            for i in 0..c {
                let val = v + i;
                self.seeds.push(val);
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Line {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl Line {
    pub fn get_destination(&self, source: u64) -> u64 {
        let diff = i64::abs((source as i64) - (self.source_range_start as i64)) as u64;
        if source < self.source_range_start + self.range_length && source >= self.source_range_start
        {
            return self.destination_range_start + diff;
        }

        source
    }
}

pub fn solve(data: &str) -> Result<u32, &'static str> {
    let mut lowest_location = u64::MAX;
    // let chunks = vec![];
    let mut queue = VecDeque::from(data.lines().collect::<Vec<&str>>());

    let mut almanac = Almanac {
        seeds: vec![],
        seed_to_soil: vec![],
        soil_to_fertilizer: vec![],
        fertilizer_to_water: vec![],
        humidity_to_location: vec![],
        light_to_temperature: vec![],
        temperature_to_humidity: vec![],
        water_to_light: vec![],
    };

    loop {
        let line = queue.pop_front();
        match line {
            Some(x) => {
                if x.starts_with("seeds") {
                    println!("seeds: {}", x);
                    let spl = x.split(":").collect::<Vec<&str>>();
                    almanac.set_seed_numbers(spl[1]);
                } else if x.starts_with(seed_to_soil) {
                    almanac.seed_to_soil = parse_chunks(&mut queue);
                } else if x.starts_with(soil_to_fertilizer) {
                    almanac.soil_to_fertilizer = parse_chunks(&mut queue);
                } else if x.starts_with(fertilizer_to_water) {
                    almanac.fertilizer_to_water = parse_chunks(&mut queue);
                } else if x.starts_with(water_to_light) {
                    almanac.water_to_light = parse_chunks(&mut queue);
                } else if x.starts_with(light_to_temperature) {
                    almanac.light_to_temperature = parse_chunks(&mut queue);
                } else if x.starts_with(temperature_to_humidity) {
                    almanac.temperature_to_humidity = parse_chunks(&mut queue);
                } else if x.starts_with(humidity_to_location) {
                    almanac.humidity_to_location = parse_chunks(&mut queue);
                }
            }
            None => break,
        }
    }

    println!("#{} seeds", almanac.seeds.len());

    // Perform the look up
    for (i, seed) in almanac.seeds.into_iter().enumerate() {
        // Start with seed -> soil
        let mut dst = find_destination(&almanac.seed_to_soil, seed);
        dst = find_destination(&almanac.soil_to_fertilizer, dst);
        dst = find_destination(&almanac.fertilizer_to_water, dst);
        dst = find_destination(&almanac.water_to_light, dst);
        dst = find_destination(&almanac.light_to_temperature, dst);
        dst = find_destination(&almanac.temperature_to_humidity, dst);
        dst = find_destination(&almanac.humidity_to_location, dst);

        if dst < lowest_location {
            lowest_location = dst;
        }

        if i % 1_000_000 == 0 {
            println!("Processed: {}", i)
        }
    }

    Ok(lowest_location as u32)
}

fn find_destination(data: &Vec<Line>, src: u64) -> u64 {
    let mut dst = src;
    for so in data {
        let tmp_dst = so.get_destination(src);
        if tmp_dst != src {
            dst = tmp_dst;
        }
    }

    dst
}

fn parse_chunks(queue: &mut VecDeque<&str>) -> Vec<Line> {
    let mut chunk = vec![];
    while let x = queue.pop_front() {
        match x {
            Some(l) => {
                if l.is_empty() {
                    return chunk;
                } else {
                    let numbers = get_numbers(l);
                    chunk.push(Line {
                        destination_range_start: numbers[0],
                        source_range_start: numbers[1],
                        range_length: numbers[2],
                    });
                }
            }
            None => break,
        }
    }

    chunk
}

fn get_numbers(data: &str) -> Vec<u64> {
    data.split_whitespace()
        .filter_map(|v| v.parse::<u64>().ok())
        .collect::<Vec<u64>>()
}

#[cfg(test)]
mod tests {
    use crate::part2::*;

    use super::Line;

    #[test]
    fn test_example() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

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

        let res = solve(input).unwrap();

        assert_eq!(res, 46)
    }

    #[test]
    fn test_line_logic_1() {
        let line = Line {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        };

        let res = line.get_destination(98);
        assert_eq!(res, 50)
    }
    #[test]
    fn test_line_logic_2() {
        let line = Line {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        };

        let res = line.get_destination(99);
        assert_eq!(res, 51)
    }
    #[test]
    fn test_line_logic_3() {
        let line = Line {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        };

        let res = line.get_destination(100);
        assert_eq!(res, 100)
    }
}
