use aoc_runner_derive::{aoc_generator, aoc};
use std::collections::{HashSet, HashMap};

pub struct Group {
  votes: Vec<String>
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Group> {
   input
    .split("\n\n")
    .map(|all| Group {
      votes: all.split("\n").map(|s| s.to_owned()).collect() 
    }).collect::<Vec<Group>>()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Vec<Group>) -> usize {
  let mut to_return = 0;

  let mut cache = HashSet::with_capacity(3);
  for group in input {
    for vote in &group.votes {
      for c in vote.chars() {
        cache.insert(c);
      }
    }

    to_return += cache.len();
    cache.clear();
  }
  
  to_return
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Vec<Group>) -> usize {
  let mut to_return = 0;

  let mut cache = HashMap::with_capacity(3);
  for group in input {
    for vote in &group.votes {
      for c in vote.chars() {
        cache.insert(c, *cache.get(&c).unwrap_or(&0) + 1);
      }
    }

    for (key, value) in cache.iter() {
      if *value == group.votes.len() {
        to_return += 1;
      }
    }
    
    cache.clear();
  }
  
  to_return
}