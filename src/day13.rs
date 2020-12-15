struct Problem {
    timestamp: u32,
    bus: Vec<(usize, u32)>,
}

impl Problem {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();
        let timestamp = lines.next().unwrap().parse().unwrap();
        let bus =
            lines
                .next()
                .unwrap()
                .split(',')
                .enumerate()
                .fold(vec![], |mut acc, (index, curr)| {
                    if let Ok(n) = curr.parse::<u32>() {
                        acc.push((index, n));
                    }
                    acc
                });
        Self { timestamp, bus }
    }

    fn solve_1(&self) -> u32 {
        let mut index = 0;
        let mut min_to_wait =
            (self.bus[index].1 - self.timestamp % self.bus[index].1) % self.bus[index].1;
        let mut i = 1;
        while i < self.bus.len() && min_to_wait > 0 {
            let to_wait = (self.bus[i].1 - self.timestamp % self.bus[i].1) % self.bus[i].1;
            if to_wait < min_to_wait {
                min_to_wait = to_wait;
                index = i;
            }
            i += 1;
        }
        self.bus[index].1 * min_to_wait
    }

    /*
    i0: t % id0 = 0 => t = id0 * n0
    ik: (t + k) % idk = 0 => t + k = idk * nk

    look for a timestamp that solve all equations
    * the timestamp is a multiple of the first bus schedule so we start by looking for a multiple of the first bus schedule that satisfy the next equation (t0)
    * once this timestamp is found, timestamps that satisfy the next equations are of form (t0 + n * lcm(schedule0, schedule1))

    */
    fn solve_2(&self) -> u64 {
        let mut incr = self.bus[0].1 as u64;
        let mut timestamp = incr;
        for i in 1..self.bus.len() {
            while (timestamp as usize + self.bus[i].0) % self.bus[i].1 as usize != 0 {
                timestamp += incr;
            }
            incr = lcm(incr, self.bus[i].1 as u64);
        }
        timestamp
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/*
a * b = ppcm(a, b) * pgcd(a, b)
*/
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn run() {
    let timer = std::time::Instant::now();
    let input = std::fs::read_to_string("inputs/day13").unwrap();
    let problem = Problem::new(&input);
    println!(
        "day 13 solution 1 : {}, {}us",
        problem.solve_1(),
        timer.elapsed().as_micros()
    );
    println!(
        "day 13 solution 2 : {}, {}us",
        problem.solve_2(),
        timer.elapsed().as_micros()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let input = "939\n7,13,x,x,59,x,31,19";
        let problem = Problem::new(&input);
        assert_eq!(problem.solve_1(), 295);
    }

    #[test]
    fn test_solution_2() {
        let input = "939\n7,13,x,x,59,x,31,19";
        let problem = Problem::new(&input);
        assert_eq!(problem.solve_2(), 1068781);
    }
}
