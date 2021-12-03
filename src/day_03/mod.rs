//use crate::helpers::DayData;
use crate::AdventDay;

pub struct DayThree();

impl AdventDay for DayThree {
    fn run_part_one(&self) -> String {
        todo!()
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
}

struct FiveBitCounter {
    bit_counters: [BitCounter; 5],
}

impl FiveBitCounter {
    fn new() -> Self {
        FiveBitCounter {
            bit_counters: [
                BitCounter::new(),
                BitCounter::new(),
                BitCounter::new(),
                BitCounter::new(),
                BitCounter::new(),
            ],
        }
    }

    fn count(&mut self, bitstring: &str) {
        let bits = bitstring.chars();
        for (i, bit) in bits.enumerate() {
            self.bit_counters[i].count(bit);
        }
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
    }

    #[test]
    fn the_five_bit_counter_counts_ones_and_zeroes() {
        let mut counter = FiveBitCounter::new();
        counter.count("10000");
        counter.count("10000");
        counter.count("10000");
        counter.count("00000");
        assert_eq!(counter.bit_counters[0].ones, 3);
        assert_eq!(counter.bit_counters[0].zeroes, 1);
    }
}
