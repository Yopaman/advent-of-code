use crate::problem::Problem;
use std::ops::Range;

pub struct DayFive {}

#[derive(Debug)]
struct Converter {
    source_ranges: Vec<Range<usize>>,
    dest_ranges: Vec<Range<usize>>,
}

impl Converter {
    fn convert(&self, src: &usize) -> usize {
        for (i, sources) in self.source_ranges.iter().enumerate() {
            if sources.contains(src) {
                let range_index = src - sources.start;
                return self.dest_ranges.get(i).unwrap().start + range_index;
            }
        }
        *src
    }

    fn convert_range(&self, start: usize, end: usize) -> Vec<(usize, usize)> {
        let mut res: Vec<(usize, usize)> = vec![];
        for (i, sources) in self.source_ranges.iter().enumerate() {
            match sources {
                //  { [ } ]
                s if start < s.start && end <= s.end => {
                    let range_index = start - sources.start;
                    res.push((start, s.start - 1));
                    res.push((
                        self.dest_ranges.get(i).unwrap().start,
                        self.dest_ranges.get(i).unwrap().start + range_index,
                    ));
                }
                // [ { ] }
                s if start >= s.start && end > s.end => {
                    let range_index = start - sources.start;
                    res.push((
                        self.dest_ranges.get(i).unwrap().start + range_index,
                        self.dest_ranges.get(i).unwrap().end,
                    ));
                    res.push((s.end + 1, end));
                }
                // [ { } ]
                s if start >= s.start && end <= s.end => {
                    let range_index1 = start - s.start;
                    let range_index2 = end - s.start;
                    res.push((
                        self.dest_ranges.get(i).unwrap().start + range_index1,
                        self.dest_ranges.get(i).unwrap().start + range_index2,
                    ));
                }
                // { [ ] }
                s if start < s.start && end > s.end => {
                    res.push((start, s.start - 1));
                    res.push((s.end + 1, end));
                    res.push((
                        self.dest_ranges.get(i).unwrap().start,
                        self.dest_ranges.get(i).unwrap().end,
                    ));
                }
                _ => {
                    res.push((start, end));
                }
            };
        }
        res
    }
}

fn parse(input: String) -> (Vec<usize>, Vec<Converter>) {
    // Parse seeds list
    let mut blocks = input.split("\n\n");
    let tmp1_seeds = blocks.next().unwrap().to_string();
    let tmp2_seeds = tmp1_seeds.strip_prefix("seeds: ").unwrap();
    let seeds = tmp2_seeds
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    // Parse each converters
    let mut converters: Vec<Converter> = vec![];
    for block in blocks {
        let mut lines = block.trim().split('\n');
        lines.next();
        let mut source_ranges = vec![];
        let mut dest_ranges = vec![];
        for line in lines {
            let mut values = line.split_whitespace().map(|n| n.parse::<usize>().unwrap());
            let dest = values.next().unwrap();
            let source = values.next().unwrap();
            let length = values.next().unwrap();
            source_ranges.push(source..source + length + 1);
            dest_ranges.push(dest..dest + length + 1);
        }
        converters.push(Converter {
            source_ranges,
            dest_ranges,
        })
    }
    (seeds, converters)
}

impl Problem for DayFive {
    fn part_one(&self, input: String) {
        let (mut seeds, converters) = parse(input);
        for converter in converters {
            for seed in &mut seeds {
                *seed = converter.convert(seed);
            }
        }
        println!("{:?}", seeds.iter().min().unwrap());
    }

    fn part_two(&self, input: String) {
        let (seeds, converters) = parse(input);
        let mut seeds_range = seeds
            .chunks(2)
            .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
            .collect::<Vec<(usize, usize)>>();
        for converter in converters {
            let mut res: Vec<(usize, usize)> = vec![];
            for range in seeds_range {
                let (start, end) = range;
                let mut new_result = converter.convert_range(start, end);
                res.append(&mut new_result);
            }
            seeds_range = res;
        }
        println!("{:?}", seeds_range);
        let mut min = 9999999;
        for (a, _) in seeds_range {
            if a < min {
                min = a;
            }
        }
        println!("{:?}", min);
    }
}
