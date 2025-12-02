use anyhow::Result;
use itertools::Itertools;
use prime_factorization::{UInt};

struct IdRange {
    first: usize,
    last: usize,
}

impl IdRange {
    fn new(first: usize, last: usize) -> Self {
        Self { first, last }
    }
}


fn parse(input: &str) -> Vec<IdRange> {
    let mut ranges = Vec::new();

    let tokens = input.split(',');

    for range_string in tokens {
        let range_ints: (usize, usize) =
            range_string.split('-')
                        .map(|s| s.parse().expect("a usize"))
                        .collect_tuple()
                        .expect("a 2-tuple of usize");

        ranges.push(IdRange::new(range_ints.0, range_ints.1));
    }

    ranges
}

pub fn part_1(input: &str) -> Result<usize> {
    let ranges = parse(input);

    let mut inval_sum = 0;

    for range in ranges {
        for i in range.first..=range.last {
            let snum = i.to_string();

            if snum.len() % 2 != 0 {continue;}

            let front_half = &snum[0..snum.len()/2];
            let back_half = &snum[snum.len()/2..];

            if front_half.eq(back_half) {
                inval_sum = inval_sum + i;
            }
        }
    }

    Ok(inval_sum)
}

pub fn part_2(input: &str) -> Result<usize> {
    let ranges = parse(input);

    let mut inval_sum = 0;

    for range in ranges {
        for num in range.first..=range.last {
            if check_num_for_p2_sequenc(num) {
                inval_sum = inval_sum + num;
            }
        }
    }

    Ok(inval_sum)
}


fn check_num_for_p2_sequenc(num: usize) -> bool {
    let mut success = true;
    let snum = num.to_string();
    let len = snum.len();

    let mut factors = list_factors(len as u32);
    factors.sort();
    factors.remove(factors.len()-1); // Don't look for sequence eq to the whole string

    // each sz will go into the snum string in a whole integer increment
    for sz in factors {
        let sz = sz as usize;
        let nibble = &snum[0..sz];
        success = true;

        // Compare the first nibble to the rest
        for j in (sz..len).step_by(sz)  {
            if nibble.ne(&snum[j..j+sz]) {
                success = false;
                break;
            }
        }

        // We found a valid repeated sequence
        if success {
            break;
        }
    }

    success
}


fn list_factors<T>(number:T) -> Vec<T>
    where T: UInt
{
    let mut factors: Vec<T> = Vec::new();
    let mut i: T = 1.into();
    while i*i <= number{
        if number % i == 0.into() {
            factors.push(i);
            if i*i != number { factors.push(number / i); }
        }
        i = i + 1.into();
    }
    factors.sort();
    factors
}


#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "11-22,\
                          95-115,\
                          998-1012,\
                          1188511880-1188511890,\
                          222220-222224,\
                          1698522-1698528,\
                          446443-446449,\
                          38593856-38593862,\
                          565653-565659,\
                          824824821-824824827,\
                          2121212118-2121212124";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT).unwrap(), 1227775554);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT).unwrap(), 4174379265);
    }

    #[test]
    fn test_part_2_bad_seqs() {
            assert_eq!(check_num_for_p2_sequenc(11), true);
            assert_eq!(check_num_for_p2_sequenc(22), true);
            assert_eq!(check_num_for_p2_sequenc(99), true);
            assert_eq!(check_num_for_p2_sequenc(111), true);
            assert_eq!(check_num_for_p2_sequenc(999), true);
            assert_eq!(check_num_for_p2_sequenc(1010), true);
            assert_eq!(check_num_for_p2_sequenc(1188511885), true);
            assert_eq!(check_num_for_p2_sequenc(222222), true);
            assert_eq!(check_num_for_p2_sequenc(446446), true);
            assert_eq!(check_num_for_p2_sequenc(38593859), true);
            assert_eq!(check_num_for_p2_sequenc(565656), true);
            assert_eq!(check_num_for_p2_sequenc(824824824), true);
            assert_eq!(check_num_for_p2_sequenc(2121212121), true);
    }
}
