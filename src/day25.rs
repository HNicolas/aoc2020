const DIVIDING_VALUE: u64 = 20201227;

fn transform(subject_number: u64, loop_size: usize) -> u64 {
    let mut result = 1u64;
    for _ in 0..loop_size {
        result = (result * subject_number).rem_euclid(DIVIDING_VALUE);
    }
    result
}

fn get_loop_size(subject_number: u64, mut current_value: u64, target_value: u64) -> usize {
    for loop_size in 1.. {
        current_value = (current_value * subject_number).rem_euclid(DIVIDING_VALUE);
        if current_value == target_value {
            return loop_size;
        }
    }
    panic!("unreachable");
}

pub fn run() {
    let timer = std::time::Instant::now();
    let card = 15733400;
    let door = 6408062;
    let door_loop_size = get_loop_size(7, 1, door);
    let encryption_key = transform(card, door_loop_size);
    // let card_loop_size = get_loop_size(7, door, card) + door_loop_size;
    // let encryption_key = transform(door, card_loop_size);
    println!("day 25 solution 1 : {}, {}us", encryption_key, timer.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let card = 5764801;
        let door = 17807724;
        let card_loop_size = get_loop_size(7, 1, card);
        assert_eq!(card_loop_size, 8);
        let door_loop_size = get_loop_size(7, card, door) + card_loop_size;
        assert_eq!(door_loop_size, 11);
        let encryption_key_1 = transform(door, card_loop_size);
        let encryption_key_2 = transform(card, door_loop_size);
        assert_eq!(encryption_key_1, 14897079);
        assert_eq!(encryption_key_2, 14897079);
    }
}
