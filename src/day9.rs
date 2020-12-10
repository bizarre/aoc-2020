use aoc_runner_derive::{aoc_generator, aoc};

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
   input
    .lines()
    .map(|line| line.parse().unwrap()).collect::<Vec<u64>>()
}

const PREAMBLE_SIZE: usize = 25;

#[aoc(day9, part1)]
pub fn solve_part1(input: &Vec<u64>) -> u64 {
  for i in PREAMBLE_SIZE..input.len() {
    let item = input[i];

    let mut found = false;
    for other in i-PREAMBLE_SIZE..i {
      let other_item = input[other];
      for other2 in i-PREAMBLE_SIZE..i {
        let other2_item = input[other2];
        if other2 != other && other_item + other2_item == item {
          found = true;
        }
      }
    }

    if !found {
      return item;
    }
  }

  0
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Vec<u64>) -> u64 {
  let target = solve_part1(input);

  'outer: for i in 0..input.len()-1 {
    let item = input[i];
    let mut count = item;
    let mut cache = vec![item];

    for o in i+1..input.len()-1 {
      let o_item = input[o];
      count += o_item;
      cache.push(o_item);
      if count == target {
        break;
      } else if count > target {
        continue 'outer;
      }
    }

    if count == target {
      cache.sort();
      return cache.first().unwrap() + cache.last().unwrap();
    }
  }

  0
}