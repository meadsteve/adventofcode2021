pub struct BitCounter {
    zeroes: usize,
    ones: usize,
}

impl BitCounter {
    pub fn new() -> Self {
        BitCounter { zeroes: 0, ones: 0 }
    }

    pub fn count(&mut self, i: char) {
        match i {
            '0' => self.zeroes += 1,
            '1' => self.ones += 1,
            _ => panic!("What the binary was that???"),
        }
    }

    pub fn most(&self) -> char {
        if self.zeroes > self.ones {
            '0'
        } else {
            '1'
        }
    }

    pub fn least(&self) -> char {
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
}
