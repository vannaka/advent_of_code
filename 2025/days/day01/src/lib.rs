use core::num;

use anyhow::{Error, Result};
#[allow(unused_imports)]
use itertools::Itertools;


struct Dial {
    total_steps: usize,
    position: usize,
}

impl Dial {
    fn new() -> Self {
        Dial { position: 50, total_steps: 100 }
    }

    // Return times dial reads zero
    fn turn_left(&mut self, steps: usize) -> usize {
        let mut passed_zero = 0usize;

        for _ in 0..steps {
            if self.position == 0 {
                self.position = self.total_steps - 1;
            } else {
                self.position = self.position - 1;
            }

            // Check if newly on zero
            if self.position == 0 {
                passed_zero = passed_zero + 1;
            }
        }

        passed_zero
    }

    // Return times dial reads zero
    fn turn_right(&mut self, steps: usize) -> usize {
        let passed_zero = (self.position + steps) / self.total_steps;

        self.position = (self.position + steps) % self.total_steps;

        passed_zero
    }

    fn get_position(&self) -> usize {
        self.position
    }
}

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_1(input: &str) -> Result<usize> {
    let _parsed = parse(input);

    let mut dial = Dial::new();
    let mut num_zeros: usize = 0;

    for instruction in _parsed {
        assert!(instruction.len() >= 2);
        let instr_ch = &mut instruction.chars();
        let dir = instr_ch.nth(0).expect("instruction is missing direction");
        let step_cnt = instr_ch.collect::<String>().parse::<usize>().expect("instruction is missing step");

        match dir {
            'R' => {
                dial.turn_right(step_cnt);
            },
            'L' => {
                dial.turn_left(step_cnt);
            },
            _ => {
                return Err(Error::msg(format!("Bad dir: {dir}")));
            }
        }

        if dial.get_position() == 0 {
            num_zeros = num_zeros + 1;
        }
    }

    Ok(num_zeros)
}

pub fn part_2(input: &str) -> Result<usize> {
    let _parsed = parse(input);

    let mut dial = Dial::new();
    let mut num_zeros: usize = 0;

    for instruction in _parsed {
        assert!(instruction.len() >= 2);
        let instr_ch = &mut instruction.chars();
        let dir = instr_ch.nth(0).expect("instruction is missing direction");
        let step_cnt = instr_ch.collect::<String>().parse::<usize>().expect("instruction is missing step");

        let cnt_touched_zero =
        match dir {
            'R' => dial.turn_right(step_cnt),
            'L' => dial.turn_left(step_cnt),
            _ => {
                return Err(Error::msg(format!("Bad dir: {dir}")));
            }
        };

        num_zeros = num_zeros + cnt_touched_zero;
    }

    Ok(num_zeros)
}


#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str =
        "L68\n\
        L30\n\
        R48\n\
        L5\n\
        R60\n\
        L55\n\
        L1\n\
        L99\n\
        R14\n\
        L82";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT).unwrap(), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT).unwrap(), 6);
    }
}
