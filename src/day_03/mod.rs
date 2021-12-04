use crate::helpers::DayData;
use crate::AdventDay;

pub struct DayThree();

impl AdventDay for DayThree {
    fn run_part_one(&self) -> String {
        let mut counter = MultiBitCounter::<12>::new();
        for line in DayData::from_file_path("./data/day03.txt").lines() {
            counter.count(&line);
        }
        let answer = usize::from_str_radix(&counter.gamma_rate(), 2).unwrap()
            * usize::from_str_radix(&counter.epsilon_rate(), 2).unwrap();
        format!("Weird thingy: {}", answer)
    }

    fn run_part_two(&self) -> String {
        todo!()
    }
}

struct BitCounter {
    zeroes: usize,
    ones: usize,
}

impl BitCounter {
    fn new() -> Self {
        BitCounter { zeroes: 0, ones: 0 }
    }

    fn count(&mut self, i: char) {
        match i {
            '0' => self.zeroes += 1,
            '1' => self.ones += 1,
            _ => panic!("What the binary was that???"),
        }
    }

    fn most(&self) -> char {
        if self.zeroes > self.ones {
            '0'
        } else {
            '1'
        }
    }

    fn least(&self) -> char {
        if self.zeroes <= self.ones {
            '0'
        } else {
            '1'
        }
    }
}

impl Default for BitCounter {
    fn default() -> Self {
        Self::new()
    }
}

trait PartOneSolver {
    fn gamma_rate(&self) -> String;
    fn epsilon_rate(&self) -> String;
}

struct MultiBitCounter<const SIZE: usize> {
    bit_counters: [BitCounter; SIZE],
}

impl MultiBitCounter<5> {
    fn new() -> Self {
        let counters: [BitCounter; 5] = Default::default();
        MultiBitCounter {
            bit_counters: counters,
        }
    }
}

impl MultiBitCounter<12> {
    fn new() -> Self {
        let counters: [BitCounter; 12] = Default::default();
        MultiBitCounter {
            bit_counters: counters,
        }
    }
}

impl<const SIZE: usize> MultiBitCounter<SIZE> {
    fn count(&mut self, bitstring: &str) {
        let bits = bitstring.chars();
        for (i, bit) in bits.enumerate() {
            self.bit_counters[i].count(bit);
        }
    }
}

impl<const SIZE: usize> PartOneSolver for MultiBitCounter<SIZE> {
    fn gamma_rate(&self) -> String {
        self.bit_counters.iter().map(|b| b.most()).collect()
    }

    fn epsilon_rate(&self) -> String {
        self.bit_counters.iter().map(|b| b.least()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bit_counter_counts_ones_and_zeroes() {
        let mut counter = BitCounter::new();
        counter.count('1');
        counter.count('1');
        counter.count('1');
        counter.count('0');
        assert_eq!(counter.ones, 3);
        assert_eq!(counter.zeroes, 1);
        assert_eq!(counter.most(), '1');
        assert_eq!(counter.least(), '0');
    }

    #[test]
    fn the_five_bit_counter_counts_ones_and_zeroes() {
        let mut counter = MultiBitCounter::<5>::new();
        counter.count("10000");
        counter.count("10000");
        counter.count("10000");
        counter.count("00000");
        assert_eq!(counter.bit_counters[0].ones, 3);
        assert_eq!(counter.bit_counters[0].zeroes, 1);
    }

    #[test]
    fn the_five_bit_counter_does_the_gamma_epsilon_thingy() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let mut counter = MultiBitCounter::<5>::new();
        for bits in input {
            counter.count(bits);
        }
        assert_eq!(counter.gamma_rate(), "10110");
        assert_eq!(counter.epsilon_rate(), "01001");
    }
}
