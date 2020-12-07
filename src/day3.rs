fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect()).collect()
}

fn get_trees_for_slope(map: &Vec<Vec<char>>, right: u8, down: u8) -> u32 {
    let mut row_index = down as usize;
    let mut column_index = right as usize;
    let mut trees = 0;
    while row_index < map.len() {
        if map[row_index][column_index] == '#' {
            trees += 1;
        }
        column_index = (column_index + right as usize) % map[row_index].len();
        row_index += down as usize;
    }
    trees
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/day3").unwrap();
    let map = parse_input(&input);
    let slope1 = get_trees_for_slope(&map, 1, 1);
    let slope2 = get_trees_for_slope(&map, 3, 1);
    let slope3 = get_trees_for_slope(&map, 5, 1);
    let slope4 = get_trees_for_slope(&map, 7, 1);
    let slope5 = get_trees_for_slope(&map, 1, 2);
    println!("day 3 solution 1 : {}", slope2);
    println!("day 3 solution 2 : {}", slope1 * slope2 * slope3 * slope4 * slope5); 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_trees_for_slope() {
        let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        assert_eq!(get_trees_for_slope(&parse_input(input), 3, 1), 7);
    }
}
