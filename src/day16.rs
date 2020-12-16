use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
struct ValidRange(u16, u16);

impl ValidRange {
    fn contains(&self, value: u16) -> bool {
        value >= self.0 && value <= self.1
    }

    fn overlap(&self, other: &Self) -> bool {
        other.1 >= self.0 && other.0 <= self.1
    }

    fn merge(&self, other: &Self) -> Self {
        Self(self.0.min(other.0), self.1.max(other.1))
    }
}

struct Input {
    fields: HashMap<String, Vec<ValidRange>>,
    ticket: Vec<u16>,
    nearby_tickets: Vec<Vec<u16>>,
    valid_ranges: Vec<ValidRange>,
}

impl Input {
    fn new(input: &str) -> Self {
        let mut parts = input.split("\n\n");
        let fields = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let mut split = line.split(": ");
                let field_name = split.next().unwrap().to_owned();
                let valid_ranges = split
                    .next()
                    .unwrap()
                    .split(" or ")
                    .map(|range| {
                        let mut numbers = range.split('-');
                        let start = numbers.next().unwrap().parse::<u16>().unwrap();
                        let end = numbers.next().unwrap().parse::<u16>().unwrap();
                        ValidRange(start, end)
                    })
                    .collect::<Vec<_>>();
                (field_name, valid_ranges)
            })
            .collect::<HashMap<_, _>>();
        let ticket = parts
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .next()
            .unwrap()
            .split(',')
            .map(|number| number.parse::<u16>().unwrap())
            .collect::<Vec<_>>();
        let nearby_tickets = parts
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(|line| {
                line.split(',')
                    .map(|number| number.parse::<u16>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let valid_ranges = merge_ranges(
            &fields
                .iter()
                .map(|(_, ranges)| ranges)
                .flatten()
                .copied()
                .collect::<Vec<_>>(),
        );
        Self {
            fields,
            ticket,
            nearby_tickets,
            valid_ranges,
        }
    }

    // better solution could be to merge ranges and to filter values
    fn solve_1(&self) -> u32 {
        self.nearby_tickets
            .iter()
            .flatten()
            .fold(0u32, |acc, &value| {
                if self.valid_ranges.iter().any(|range| range.contains(value)) {
                    acc
                } else {
                    acc + value as u32
                }
            })
    }

    fn solve_2(&self) -> u64 {
        let mut valid_tickets = self
            .nearby_tickets
            .iter()
            .filter(|&ticket| {
                ticket
                    .iter()
                    .all(|&value| self.valid_ranges.iter().any(|range| range.contains(value)))
            })
            .collect::<Vec<_>>();
        valid_tickets.push(&self.ticket);

        let mut valid_fields_at_index = vec![
            self.fields
                .iter()
                .map(|(name, _)| name)
                .collect::<HashSet<_>>();
            self.fields.len()
        ];

        let departure_field_count = self.fields.iter().fold(0, |acc, (curr, _)| {
            if curr.starts_with("departure") {
                acc + 1
            } else {
                acc
            }
        });

        for ticket in valid_tickets.iter() {
            for index in 0..valid_fields_at_index.len() {
                let valid_fields = &mut valid_fields_at_index[index];
                if valid_fields.len() == 1 {
                    continue;
                }

                let invalid_fields = valid_fields
                    .iter()
                    .filter(|&&field| {
                        !self
                            .fields
                            .get(field)
                            .unwrap()
                            .iter()
                            .any(|range| range.contains(ticket[index]))
                    })
                    .copied()
                    .collect::<Vec<_>>();
                for invalid_field in invalid_fields {
                    valid_fields.remove(invalid_field);
                }

                if valid_fields.len() == 1 {
                    let mut to_remove = vec![];
                    to_remove.push((index, *valid_fields.iter().next().unwrap()));
                    while to_remove.len() > 0 {
                        let mut next_to_remove = vec![];
                        for &(i, field) in &to_remove {
                            for j in 0..valid_fields_at_index.len() {
                                if j != i && valid_fields_at_index[j].contains(field) {
                                    valid_fields_at_index[j].remove(field);
                                    if valid_fields_at_index[j].len() == 1 {
                                        next_to_remove.push((
                                            j,
                                            *valid_fields_at_index[j].iter().next().unwrap(),
                                        ));
                                    }
                                }
                            }
                        }
                        to_remove = next_to_remove;
                    }
                }
            }

            let departure_fields = valid_fields_at_index
                .iter()
                .enumerate()
                .filter(|&(_, set)| {
                    set.len() == 1 && set.iter().next().unwrap().starts_with("departure")
                })
                .map(|(index, _)| index)
                .collect::<Vec<_>>();
            if departure_fields.len() == departure_field_count {
                return departure_fields
                    .iter()
                    .fold(1, |acc, &index| acc * self.ticket[index] as u64);
            }
        }
        panic!("no solution found");
    }
}

fn merge_ranges(ranges: &Vec<ValidRange>) -> Vec<ValidRange> {
    let mut old_length = ranges.len();
    let mut merged = merge_ranges_once(ranges);
    while merged.len() < old_length {
        old_length = merged.len();
        merged = merge_ranges_once(&merged);
    }
    merged
}

fn merge_ranges_once(ranges: &Vec<ValidRange>) -> Vec<ValidRange> {
    let mut vec: Vec<ValidRange> = vec![];
    for range in ranges {
        if let Some((index, overlapping_range)) = vec
            .iter()
            .enumerate()
            .find(|(_, stored_range)| stored_range.overlap(range))
        {
            vec[index] = range.merge(overlapping_range);
        } else {
            vec.push(range.to_owned());
        }
    }
    vec
}

pub fn run() {
    let timer = std::time::Instant::now();
    let input = std::fs::read_to_string("inputs/day16").unwrap();
    let input = Input::new(&input);
    println!(
        "day 16 solution 1 : {}, {}us",
        input.solve_1(),
        timer.elapsed().as_micros()
    );
    println!(
        "day 16 solution 2 : {}, {}us",
        input.solve_2(),
        timer.elapsed().as_micros()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let input = "class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\n\nyour ticket:\n7,1,14\n\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12";
        let input = Input::new(&input);
        assert_eq!(input.solve_1(), 71);
    }

    #[test]
    fn test_solution_2() {
        let input = "departure class: 0-1 or 4-19\nrow: 0-5 or 8-19\nseat: 0-13 or 16-19\n\nyour ticket:\n11,12,13\n\nnearby tickets:\n3,9,18\n15,1,5\n5,14,9";
        let input = Input::new(&input);
        assert_eq!(input.solve_2(), 12);
    }
}
