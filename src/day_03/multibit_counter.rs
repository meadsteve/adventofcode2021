use crate::day_03::bit_counter::BitCounter;
use crate::day_03::PartOneSolver;

pub struct MultiBitCounter<const SIZE: usize> {
    bit_counters: [BitCounter; SIZE],
}

#[allow(dead_code)]
impl MultiBitCounter<5> {
    pub fn new() -> Self {
        MultiBitCounter {
            bit_counters: Default::default(),
        }
    }
}

impl MultiBitCounter<12> {
    pub fn new() -> Self {
        MultiBitCounter {
            bit_counters: Default::default(),
        }
    }
}

impl<const SIZE: usize> MultiBitCounter<SIZE> {
    pub fn count(&mut self, bitstring: &str) {
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
    use crate::day_03::*;

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
