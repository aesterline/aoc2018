use std::io::{BufRead, Result};

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
    fn frequency_for_day_1_input() {
        let input_file = File::open("test_data/day_1_input.txt").unwrap();
        let file = BufReader::new(&input_file);
        assert_eq!(582, frequency(file).unwrap())
    }
}
