use anyhow::Result;
#[allow(unused_imports)]
use itertools::Itertools;
use range_set_blaze::RangeSetBlaze;

pub fn parse(input: &str) -> (RangeSetBlaze<u64>, Vec<u64>) {
    let mut processing_ranges = true;
    let mut range_list = Vec::new();
    let mut ids = Vec::new();

    for line in input.lines() {
        if processing_ranges {
            if line.eq("") {
                processing_ranges = false;
                continue;
            }

            let (start, end) = line.split_once('-').expect("malformed input");
            let start: u64 = start.parse().expect("malformed input");
            let end: u64 = end.parse().expect("malformed input");

            range_list.push(start..=end);
        } else {
            ids.push(line.parse().expect("invalid input"));
        }
    }

    (RangeSetBlaze::from_iter(range_list.iter()), ids)
}

pub fn part_1(input: &str) -> Result<usize> {
    let (ranges, ids) = parse(input);

    let mut valid_cnt = 0;

    for id in ids {
        if ranges.contains(id) {
            valid_cnt += 1;
        }
    }

    Ok(valid_cnt)
}

pub fn part_2(input: &str) -> Result<usize> {
    let (ranges, _) = parse(input);

    let mut valid_cnt = 0;

    for range in ranges.into_ranges() {
        valid_cnt += (range.end() + 1) - range.start();
    }

    Ok(valid_cnt as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
"3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT).unwrap(), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT).unwrap(), 14);
    }
}
