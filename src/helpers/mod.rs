use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub struct DayData(String);

impl<'a> DayData {
    pub fn from_file_path(path: &str) -> DayData {
        DayData(path.to_string())
    }

    pub fn lines(&self) -> DayDataLineIterator {
        DayDataLineIterator::new(self)
    }

    pub fn parse_comma_separated_parts<T>(&self, parse: fn(&str) -> T) -> Vec<T> {
        let first_line = self
            .lines()
            .next()
            .expect("The file should have one comma separated line");
        first_line.split(',').map(|i| parse(i)).collect()
    }
}

pub struct DayDataLineIterator<'a>(&'a DayData, Lines<BufReader<File>>);

impl<'a> DayDataLineIterator<'a> {
    fn new(data: &'a DayData) -> DayDataLineIterator {
        let f = File::open(&data.0).expect("input not found");
        let f = BufReader::new(f);
        DayDataLineIterator(data, f.lines())
    }
}

impl<'a> Clone for DayDataLineIterator<'a> {
    fn clone(&self) -> Self {
        DayDataLineIterator::new(self.0)
    }
}

impl<'a> Iterator for DayDataLineIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|s| s.unwrap())
    }
}
