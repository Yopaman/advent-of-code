use crate::inputs;
use std::cmp::min;
use std::cmp::max;
use itertools::Itertools;

struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.x1 == self.x2
    }

    fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }

    fn is_diagonal(&self) -> bool {
        self.x1 != self.x2 && self.y1 != self.y2
    }
}

fn parse_inputs() -> Vec<Line> {
    let lines = inputs::read_lines("./src/inputs/day5.txt");
    lines.iter().map(|l| {
        let parse: Vec<_> = l.split(" -> ").collect();
        let left_parse: Vec<_> = parse[0].split(",").collect();
        let right_parse: Vec<_> = parse[1].split(",").collect();
        let x1 = left_parse[0].parse::<i32>().unwrap();
        let y1 = left_parse[1].parse::<i32>().unwrap();
        let x2 = right_parse[0].parse::<i32>().unwrap();
        let y2 = right_parse[1].parse::<i32>().unwrap();
        Line{x1,y1,x2,y2}
    }).collect()
}

fn get_all_points(lines: Vec<Line>) -> Vec<(i32, i32)> {
    let mut points_vec: Vec<(i32, i32)> = vec!();
    lines.iter().for_each(|l| {
        if l.is_horizontal() {
            let range = if l.x1 < l.x2 {
                l.x1 ..= l.x2
            } else {
                l.x2 ..= l.x1
            };
            range.for_each(|x| points_vec.push((x, l.y1)));
        } else if l.is_vertical() {
            let range = if l.y1 < l.y2 {
                l.y1 ..= l.y2
            } else {
                l.y2 ..= l.y1
            };
            range.for_each(|y| points_vec.push((l.x1, y)));
        } else if l.is_diagonal() {
            if l.x1 <= l.x2 && l.y1 <= l.y2 {
                let mut x = l.x1;
                let mut y = l.y1;
                while x <= l.x2 && y <= l.y2 {
                    points_vec.push((x, y));
                    x += 1;
                    y += 1;
                }
            } else if l.x1 <= l.x2 && l.y1 >= l.y2 {
                let mut x = l.x1;
                let mut y = l.y1;
                while x <= l.x2 && y >= l.y2 {
                    points_vec.push((x, y));
                    x += 1;
                    y -= 1;
                }
            } else if l.x1 >= l.x2 && l.y1 <= l.y2 {
                let mut x = l.x1;
                let mut y = l.y1;
                while x >= l.x2 && y <= l.y2 {
                    points_vec.push((x, y));
                    x -= 1;
                    y += 1;
                }
            } else if l.x1 >= l.x2 && l.y1 >= l.y2 {
                let mut x = l.x1;
                let mut y = l.y1;
                while x >= l.x2 && y >= l.y2 {
                    points_vec.push((x, y));
                    x -= 1;
                    y -= 1;
                }
            }
            
        }
    });
    points_vec
}

pub fn part1() {
    let lines = parse_inputs();
    let mut points = get_all_points(lines);
    points.sort();
    let count = points.into_iter().dedup_with_count().fold(0, |acc, p| if p.0 > 1 { acc + 1 } else { acc });
    println!("result : {}", count);
}