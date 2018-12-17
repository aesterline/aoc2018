pub mod day1;
pub mod day2;

#[cfg(test)]
mod tests {
  use super::day1::*;
  use super::day2::*;

  use std::fs::File;
  use std::io::{BufRead, BufReader};

  #[test]
  fn frequency_for_day_1_input() {
    let input_file = File::open("test_data/day_1_input.txt").unwrap();
    let file = BufReader::new(&input_file);
    assert_eq!(582, frequency(file).unwrap())
  }

  #[test]
  fn repeated_frequences_for_day_1_input() {
    let input_file = File::open("test_data/day_1_input.txt").unwrap();
    let file = BufReader::new(&input_file);
    assert_eq!(488, first_repeated_frequency(file).unwrap())
  }

  #[test]
  fn checksum_for_day_2_input() {
    let input_file = File::open("test_data/day_2_input.txt").unwrap();
    let file = BufReader::new(&input_file);
    let ids = file.lines().map(|id| Id::scan(id.unwrap().as_str()));

    assert_eq!(4920, Id::checksum(ids))
  }
}
