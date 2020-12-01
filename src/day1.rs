use aoc_runner_derive::{aoc_generator, aoc};
use itertools::Itertools;

type Expense = u32;
const GOAL: Expense = 2020;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Expense> {
  input.lines()
    .map(|d| d.parse().unwrap())
    .collect()
}

#[aoc(day1, part1, for_loop)]
pub fn solve_part1_for_loop(expenses: &Vec<Expense>) -> u32 {
  for i in expenses {
    for x in expenses {
      if i + x == GOAL {
        return i * x
      }
    }
  }

  unreachable!()
}

#[aoc(day1, part1, recursion)]
pub fn solve_part1_recursion(expenses: &Vec<Expense>) -> u32 {
  fn recurse(expenses: &mut Vec<Expense>) -> u32 {
    let expense = expenses.pop().ok_or(0).unwrap();

    for other in expenses.iter() {
      if *other + expense == GOAL {
        return *other * expense
      }
    }

    recurse(expenses)
  }

  recurse(&mut expenses.to_owned())
}

#[aoc(day1, part1, binary_search)]
pub fn solve_part1_binary_search(expenses: &Vec<Expense>) -> u32 {
  let sorted: Vec<u32> = expenses.into_iter().map(|d| *d).sorted().collect();
  
  let length = sorted.len();
  for value in sorted.iter() {
    let difference: u32 = GOAL - value;
    
    let mut low: usize = 0;
    let mut mid: usize;
    let mut high: usize = length - 1;

    while low < high {
      mid = high + low;
      if sorted[mid] < difference {
        low = mid + 1;
      } else if sorted[mid] > difference {
        high = mid - 1;
      } else {
        return sorted[mid] * value;
      }
      
    }

  }

  unreachable!()
}


#[aoc(day1, part2, for_loop)]
pub fn solve_part2_for_loop(expenses: &Vec<Expense>) -> u32 {
  for i in expenses {
    for x in expenses {
      for h in expenses {
        if i + x + h == GOAL {
          return i * x * h
        }
      }
    }
  }

  unreachable!()
}