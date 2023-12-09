use crate::problem::Problem;

pub struct DaySix {}

fn get_number_of_ways(time: usize, record: usize) -> usize {
    let mut sum = 0;
    for i in 0..time {
        if i * (time - i) > record {
            sum += 1;
        }
    }
    sum
}

impl Problem for DaySix {
    fn part_one(&self, input: String) {
        let mut lines = input.split('\n');
        let first_split = lines.next().unwrap().chars().skip(6).collect::<String>();
        let times = first_split
            .split_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let second_split = lines.next().unwrap().chars().skip(10).collect::<String>();
        let records = second_split
            .split_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut product = 0;
        for (i, time) in times.iter().enumerate() {
            let record = &records.get(i).unwrap();
            let number = get_number_of_ways(*time, **record);
            if product == 0 {
                product = number;
            } else {
                product *= number;
            }
        }
        println!("{}", product)
    }

    fn part_two(&self, input: String) {
        let mut lines = input.split('\n');
        let mut time = lines.next().unwrap().chars().skip(6).collect::<String>();
        time.retain(|c| !c.is_whitespace());
        let time_nbr = time.parse::<usize>().unwrap();
        let mut record = lines.next().unwrap().chars().skip(10).collect::<String>();
        record.retain(|c| !c.is_whitespace());
        let record_nbr = record.parse::<usize>().unwrap();
        println!("{}", get_number_of_ways(time_nbr, record_nbr));
    }
}
