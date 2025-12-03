use core::num;

use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_1(input: &str) -> Result<usize> {
    let _parsed = parse(input);
    let mut sum: usize = 0;

    for bank in _parsed {
        let mut first = '0';
        let mut first_n = 0usize;
        let mut second = '0';

        // Find first largest
        for (n, c) in bank.chars().enumerate() {
            if n >= bank.len()-1 {
                break;
            }

            if c > first {
                first = c;
                first_n = n;
            }
        }

        // Find largest in the bank after the position of the first
        for c in bank.chars().skip(first_n+1) {
            if c > second {
                second = c;
            }
        }

        let val: usize = format!("{}{}", first, second).parse().expect("NaN in input");

        sum += val;
    }

    Ok(sum)
}

pub fn part_2(input: &str) -> Result<usize> {
    let _parsed = parse(input);
    let mut sum = 0;

    for bank in _parsed {
        sum += find_largest_number(bank);
    }

    Ok(sum)
}


// bank = 987654321111111
// bank.len() = 15
// res_len = 12
// cnt_found = 0
// cursor = 0

// Find largest number in rang:
//      [cursor..bank.len() - (res_len - cnt_found)]

fn find_largest_number(bank: &str) -> usize {
    let mut num_v = do_find(bank, 12, 0);
    num_v.reverse();

    let num_s: String = num_v.iter().collect();
    let num = num_s.parse().expect("non number char");

    num
}

fn do_find(bank: &str, cnt_left: usize, cursor: usize) -> Vec<char> {

    // Base case 1
    if cnt_left == 0 {
        Vec::new()
    }
    else {
        let mut largest = '0';
        let mut largest_n = 0;

        // Find largest number in
        //  [cursor..bank.len() - (res_len - cnt_found)]
        for (n, c) in bank.chars().enumerate().skip(cursor) {
            if c > largest {
                largest = c;
                largest_n = n+1;
            }

            if bank.len() - n == cnt_left && cnt_left != 1 {
                break;
            }
        }

        let mut res = do_find(bank, cnt_left-1, largest_n);
        res.push(largest);
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT).unwrap(), 357);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT).unwrap(), 3121910778619);
    }

    #[test]
    fn test_find_largest_number() {
        assert_eq!(find_largest_number("987654321111111"), 987654321111);
        assert_eq!(find_largest_number("811111111111119"), 811111111119);
        assert_eq!(find_largest_number("234234234234278"), 434234234278);
        assert_eq!(find_largest_number("818181911112111"), 888911112111);
    }


}
