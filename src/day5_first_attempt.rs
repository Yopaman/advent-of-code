use crate::inputs;
use std::cmp::max;
use std::cmp::min;

// Doesn't work because it counts the number of times each point is collapsed so some points can be counted multiple times.

#[derive(Debug)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

fn count_points_same_orientation(min1: i32, max1: i32, min2: i32, max2: i32) -> i32 {
    if max1 < min2 || max2 < min1 {
        return 0;
    } else if min1 <= min2 && max1 <= max2 {
        return (max1 - min2).abs();
    } else if min1 >= min2 && max1 <= max2 {
        return (max1 - min1).abs();
    } else if min1 >= min2 && max1 >= max2 {
        return (max2 - min1).abs();
    } else if min1 <= min2 && max1 >= max2 {
        return (max2 - min2).abs();
    }
    return 0;
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.x1 == self.x2
    }

    fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }

    fn count_colliding_points(&self, second_line: &Line) -> i32 {
        if second_line.is_vertical() && self.is_horizontal() {
            let min1 = min(self.x1, self.x2);
            let max1 = max(self.x1, self.x2);
            let min2 = min(second_line.y1, second_line.y2);
            let max2 = max(second_line.y1, second_line.y2);

            if (self.y1 >= min2 && self.y1 <= max2) && (min1 <= second_line.x1 && max1 >= second_line.x1) {
                return 1;
            }
        } else if second_line.is_horizontal() && self.is_vertical() {
            let min1 = min(self.y1, self.y2);
            let max1 = max(self.y1, self.y2);
            let min2 = min(second_line.x1, second_line.x2);
            let max2 = max(second_line.x1, second_line.x2);

            if (self.x1 >= min2 && self.x1 <= max2) && (min1 <= second_line.y1 && max1 >= second_line.y1){
                return 1;
            }
        } else {
            if self.is_horizontal() && second_line.is_horizontal() {
                if self.y1 == second_line.y1 {
                    let min1 = min(self.x1, self.x2);
                    let max1 = max(self.x1, self.x2) + 1;
                    let min2 = min(second_line.x1, second_line.x2);
                    let max2 = max(second_line.x1, second_line.x2) + 1;
                    return count_points_same_orientation(min1, max1, min2, max2);
                }
            } else if self.is_vertical() && second_line.is_vertical() {
                // println!("{},{} -> {},{} et {},{} -> {},{} ", self.x1, self.y1, self.x2, self.y2, second_line.x1, second_line.y1, second_line.x2, second_line.y2);
                if self.x1 == second_line.x1 {
                    let min1 = min(self.y1, self.y2);
                    let max1 = max(self.y1, self.y2) + 1;
                    let min2 = min(second_line.y1, second_line.y2);
                    let max2 = max(second_line.y1, second_line.y2) + 1;
                    return count_points_same_orientation(min1, max1, min2, max2);
                }   
            }
        }
        return 0;
    }
}

fn parse_inputs() -> Vec<Line> {
    let lines = inputs::read_lines("./src/inputs/day5_test.txt");
    let mut lines_vec = vec!();
    for e in lines {
        let split: Vec<_> = e.split(" -> ").collect();
        let first_point: Vec<_> = split[0].split(",").collect();
        let second_point: Vec<_> = split[1].split(",").collect();
        let x1 = first_point[0].parse::<i32>().unwrap();
        let y1 = first_point[1].parse::<i32>().unwrap();
        let x2 = second_point[0].parse::<i32>().unwrap();
        let y2 = second_point[1].parse::<i32>().unwrap();
        lines_vec.push(Line{x1,y1,x2,y2})
    }
    lines_vec
}

pub fn part1() {
    let lines = parse_inputs();
    let count = lines.iter().enumerate().fold(0, |acc, (i,e)| 
        acc + &lines[i+1..].iter().fold(0, |acc2, e2| {
            //println!("nomber of overlap for {:?} with {:?} : {}",e,e2,e.count_colliding_points(e2));
            acc2 + e.count_colliding_points(e2)
        }));
    println!("result : {}", count);
}