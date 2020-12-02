use aoc_runner_derive::{aoc_generator, aoc};
use itertools::Itertools;

#[derive(Debug)]
pub struct Policy {
  character: char,
  min: u8,
  max: u8
}

#[derive(Debug)]
pub struct Item {
  password: String,
  policy: Policy
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Item> {
  input.lines()
    .map(|d| {
      let chars: Vec<char> = d.chars().collect();
      let min: u8;
      let max: u8;

      let mut index: usize = 0;
      let mut buffer = Vec::new();
      while chars[index] != '-' {
        buffer.push(chars[index]);
        index += 1;
      }

      index += 1;

      min = buffer.into_iter().collect::<String>().parse().unwrap();

      let mut buffer = Vec::new();
      while chars[index] != ' ' {
        buffer.push(chars[index]);
        index += 1;
      }
      index += 1;

      max = buffer.into_iter().collect::<String>().parse().unwrap();

      let policy = Policy {
        character: chars[index],
        max: max,
        min: min
      };

      index += 3;

      Item {
        policy: policy,
        password: chars[index..chars.len()].into_iter().collect()
      }
    }).collect()
}

#[aoc(day2, part1, matches)]
pub fn solve_part1_matches(items: &Vec<Item>) -> u32 {
  let mut valid: u32 = 0;

  for item in items {
    let count = item.password.matches(item.policy.character).count() as u8;
    if count >= item.policy.min && count <= item.policy.max {
      valid += 1;
    }
  }

  valid
}

#[aoc(day2, part1, counter)]
pub fn solve_part1_counter(items: &Vec<Item>) -> u32 {
  let mut valid: u32 = 0;

  for item in items {
    let mut count = 0;
    for c in item.password.chars() {
      if c == item.policy.character {
        count += 1;
      }
    }

    if count >= item.policy.min && count <= item.policy.max {
      valid += 1;
    }
  }

  valid
}

#[aoc(day2, part1, replace)]
pub fn solve_part1_replace(items: &Vec<Item>) -> u32 {
  let mut valid: u32 = 0;

  for item in items {
    let count = (item.password.len() - str::replace(&item.password, item.policy.character, "").len()) as u8;
    
    if count >= item.policy.min && count <= item.policy.max {
      valid += 1;
    }
  }

  valid
}

#[aoc(day2, part2)]
pub fn solve_part2(items: &Vec<Item>) -> u32 {
  let mut valid: u32 = 0;

  for item in items {
    let max = item.policy.max;
    let min = item.policy.min;
    let password = &item.password;
    let length = (password.len() as u8);
    let target = item.policy.character;
    let chars: Vec<char> = password.chars().into_iter().collect();
    let contains_first = chars.len() > (max - 1) as usize && chars[(max - 1) as usize] == target;
    let contains_second = chars.len() > (min - 1) as usize && chars[(min - 1) as usize] == target;

    if contains_first ^ contains_second {
      valid += 1;
    }
  }

  valid
}