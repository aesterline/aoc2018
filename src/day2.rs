use std::collections::HashMap;

#[derive(Default)]
pub struct Id {
    contains_a_pair: bool,
    contains_three_of_a_kind: bool,
}

impl Id {
    pub fn scan(id: &str) -> Id {
        id.chars()
            .fold(HashMap::new(), |mut counts, c| {
                counts.entry(c).and_modify(|count| *count += 1).or_insert(1);
                counts
            })
            .values()
            .fold(Id::default(), |mut scanner, val| {
                match *val {
                    2 => scanner.contains_a_pair = true,
                    3 => scanner.contains_three_of_a_kind = true,
                    _ => (),
                }
                scanner
            })
    }

    pub fn checksum<I>(ids: I) -> i32
    where
        I: Iterator<Item = Id>,
    {
        let mut number_of_pairs = 0;
        let mut number_of_three_of_a_kind = 0;

        for id in ids {
            if id.contains_a_pair {
                number_of_pairs += 1;
            }

            if id.contains_three_of_a_kind {
                number_of_three_of_a_kind += 1;
            }
        }

        println!("{} * {}", number_of_pairs, number_of_three_of_a_kind);

        number_of_pairs * number_of_three_of_a_kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_does_not_repeat_letters() {
        let scanner = Id::scan("abc");

        assert_eq!(scanner.contains_a_pair, false);
        assert_eq!(scanner.contains_three_of_a_kind, false);
    }

    #[test]
    fn input_repeats_one_letter() {
        let scanner = Id::scan("aca");

        assert_eq!(scanner.contains_a_pair, true);
        assert_eq!(scanner.contains_three_of_a_kind, false);
    }

    #[test]
    fn only_count_one_repeat() {
        let scanner = Id::scan("acac");

        assert_eq!(scanner.contains_a_pair, true);
        assert_eq!(scanner.contains_three_of_a_kind, false);
    }

    #[test]
    fn input_repeats_multiple_letters() {
        let scanner = Id::scan("bababc");

        assert_eq!(scanner.contains_a_pair, true);
        assert_eq!(scanner.contains_three_of_a_kind, true);
    }

    #[test]
    fn checksum_can_be_calculated() {
        let ids = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];

        assert_eq!(12, Id::checksum(ids.iter().map(|id| Id::scan(id))));
    }
}
