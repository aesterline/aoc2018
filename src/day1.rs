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

struct BufReadCycle<B> {
    buf: B,
}

impl<B: BufRead + Seek> Iterator for BufReadCycle<B> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut line = String::new();
        match self.buf.read_line(&mut line) {
            Ok(0) => {
                if self.buf.seek(SeekFrom::Start(0)).is_err() {
                    return None;
                }
                self.next()
            }
            Ok(_) => Some(line.trim_end().to_string()),
            Err(_) => None,
        }
    }
}

pub fn first_repeated_frequency<T>(changes: T) -> Result<i64>
where
    T: BufRead + Seek,
{
    let mut seen_frequences: HashSet<i64> = HashSet::new();
    let mut current_frequency = 0;
    let lines = BufReadCycle { buf: changes };

    for line in lines {
        if seen_frequences.contains(&current_frequency) {
            return Ok(current_frequency);
        }
        seen_frequences.insert(current_frequency);
        current_frequency += line.parse::<i64>().unwrap();
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

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
    fn input_is_cycled() {
        let buf = Cursor::new(b"foo\nbar");
        let mut iter = BufReadCycle { buf };

        assert_eq!(Some(String::from("foo")), iter.next());
        assert_eq!(Some(String::from("bar")), iter.next());
        assert_eq!(Some(String::from("foo")), iter.next());
        assert_eq!(Some(String::from("bar")), iter.next());
    }
}
