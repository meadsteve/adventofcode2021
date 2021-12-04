use crate::day_03::bit_counter::BitCounter;

pub fn find_oxygen_rating(strings: Vec<String>) -> String {
    filter_using_counter(strings, BitCounter::most)
}

pub fn find_scrubber_rating(strings: Vec<String>) -> String {
    filter_using_counter(strings, BitCounter::least)
}

fn filter_using_counter(strings: Vec<String>, f: fn(&BitCounter) -> char) -> String {
    let mut strings = strings;
    for position in 0..12 {
        let mut counter = BitCounter::new();
        for s in &strings {
            counter.count(char_at_pos(s, position))
        }
        let filter_char = f(&counter);
        strings = strings
            .iter()
            .filter(|s| char_at_pos(s, position) == filter_char)
            .cloned()
            .collect();
        if strings.len() == 1 {
            return strings.pop().unwrap();
        }
    }
    panic!("Err this shouldn't happen");
}

fn char_at_pos(s: &str, pos: usize) -> char {
    s.chars().nth(pos).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day_03::ratings::{find_oxygen_rating, find_scrubber_rating};

    #[test]
    fn test_find_oxygen_rating() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let rating = find_oxygen_rating(input);
        assert_eq!(rating, "10111");
    }

    #[test]
    fn test_find_scrubber_rating() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let rating = find_scrubber_rating(input);
        assert_eq!(rating, "01010");
    }
}
