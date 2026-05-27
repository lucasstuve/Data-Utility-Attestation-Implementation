extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub fn check_list(mut list: Vec<(u32, u32)>) -> bool {
    let mut l_accumulator: u32 = 0;
    list.sort_unstable();
    let mut previous_pair: (u32, u32) = list[0];
    if list.is_empty() {
        return false;
    }
    l_accumulator += pair_length(previous_pair.0, previous_pair.1);

    let expected_length = expected_length(&list);
    let approximate_delimiter_shift: u32 = list.len() as u32;
    let mut has_doubles: bool = false;
    let mut overlaps: bool = false;
    let mut has_gap: bool = false;

    for &l in list.iter().skip(1) {
        if l.1 <= l.0 {
            return false;
        }

        if l == previous_pair {
            has_doubles = true;
        }

        if previous_pair.1 > l.0 {
            overlaps = true;
        }
        if previous_pair.1 < l.0 {
            has_gap = true;
        }

        l_accumulator = l_accumulator + pair_length(l.0, l.1);
        previous_pair = l;
    }

    return !(l_accumulator + approximate_delimiter_shift <= expected_length)
        && !has_doubles
        && !overlaps
        && !has_gap;
}

fn pair_length(start: u32, end: u32) -> u32 {
    return end - start;
}
fn expected_length(list: &Vec<(u32, u32)>) -> u32 {
    let first_index = list[0].0;
    let list_length = list.len();
    let last_index = list[list_length - 1].1;

    return last_index - first_index;
}

/*
#[cfg(test)]
mod list_validator_tests {
    use super::*;

    #[test]
    fn accepts_continuous_ranges() {
        assert!(check_list(vec![(0, 3), (3, 6), (6, 10)]));
    }

    #[test]
    fn accepts_unsorted_but_continuous_ranges() {
        assert!(check_list(vec![(6, 10), (0, 3), (3, 6)]));
    }

    #[test]
    fn rejects_gap() {
        assert!(!check_list(vec![(0, 3), (4, 6)]));
    }

    #[test]
    fn rejects_overlap() {
        assert!(!check_list(vec![(0, 5), (3, 8)]));
    }

    #[test]
    fn rejects_duplicate() {
        assert!(!check_list(vec![(0, 3), (0, 3)]));
    }

    #[test]
    fn rejects_empty_list() {
        assert!(!check_list(vec![]));
    }

    #[test]
    fn rejects_invalid_range() {
        assert!(!check_list(vec![(3, 3)]));
        assert!(!check_list(vec![(5, 3)]));
    }

    #[test]
    fn accepts_single_valid_range() {
        assert!(check_list(vec![(2, 8)]));
    }
}

 */
