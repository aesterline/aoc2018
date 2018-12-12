use std::collections::HashSet;
use std::io::{BufRead, Result, Seek, SeekFrom};

pub fn frequency<T>(changes: T) -> Result<i64>
where
    T: BufRead,
{
    // changes.lines().fold(Ok(0), |result, change| {
    //     Ok(result? + change?.parse::<i64>().unwrap())
    // })

    let mut result = 0;
    for line in changes.lines() {
        result += line?.parse::<i64>().unwrap()
    }
    Ok(result)
}

pub fn first_repeated_frequency<T>(mut changes: T) -> Result<i64>
where
    T: BufRead + Seek,
{
    let mut seen_frequences: HashSet<i64> = HashSet::new();
    let mut current_frequency = 0;
    let mut line = String::new();

    while !seen_frequences.contains(&current_frequency) {
        seen_frequences.insert(current_frequency);
        line.clear();
        if changes.read_line(&mut line)? == 0 {
            changes.seek(SeekFrom::Start(0))?;
            changes.read_line(&mut line)?;
        }
        current_frequency += line.trim_end().parse::<i64>().unwrap();
    }

    Ok(current_frequency)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, Cursor};

    #[test]
    fn parse_can_handle_positive_numbers() {
        let value = "+1".parse::<i64>().unwrap();
        assert_eq!(1, value)
    }

    #[test]
    fn parse_can_handle_negative_numbers() {
        let value = "-4".parse::<i64>().unwrap();
        assert_eq!(-4, value)
    }

    #[test]
    fn frequency_when_empty_read() {
        let input = Cursor::new(Vec::new());
        assert_eq!(0, frequency(input).unwrap());
    }

    #[test]
    fn frequency_when_changes_contains_a_single_number() {
        let input = Cursor::new(b"+1");
        assert_eq!(1, frequency(input).unwrap());
    }

    #[test]
    fn frequency_when_changes_contains_multiple_numbers() {
        let input = Cursor::new(b"+1\n+3");
        assert_eq!(4, frequency(input).unwrap());
    }

    #[test]
    fn frequency_for_day_1_input() {
        let input_file = File::open("test_data/day_1_input.txt").unwrap();
        let file = BufReader::new(&input_file);
        assert_eq!(582, frequency(file).unwrap())
    }

    #[test]
    fn repeated_frequences() {
        fn first_repeat(input: &[u8]) -> i64 {
            first_repeated_frequency(Cursor::new(input)).unwrap()
        }

        assert_eq!(0, first_repeat(b"+1\n-1"));
        assert_eq!(10, first_repeat(b"+3\n+3\n+4\n-2\n-4"));
        assert_eq!(5, first_repeat(b"-6\n+3\n+8\n+5\n-6"));
        assert_eq!(14, first_repeat(b"+7\n+7\n-2\n-7\n-4"));
    }

    #[test]
    fn repeated_frequences_for_day_1_input() {
        let input_file = File::open("test_data/day_1_input.txt").unwrap();
        let file = BufReader::new(&input_file);
        assert_eq!(488, first_repeated_frequency(file).unwrap())
    }
}
