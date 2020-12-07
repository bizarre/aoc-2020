use aoc_runner_derive::{aoc_generator, aoc};
use std::collections::{HashMap};

#[derive(Clone)]
pub struct BagContents {
  amount: u8,
  bag: String
}

#[derive(Clone)]
pub struct Bag {
  name: String,
  contents: Vec<BagContents>
}


#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, Bag> {
  let mut to_return: HashMap<String, Bag> = HashMap::new();

  for line in input.lines().collect::<Vec<&str>>() {
    let split: Vec<&str> = line.split("contain").collect();
    let name = split[0].trim().trim_end_matches("bags.").trim_end_matches("bags").trim_end_matches("bag").trim_end_matches("bag.").trim();

    if !to_return.contains_key(name) {
      to_return.insert(name.to_owned(), Bag {
        name: name.to_owned(),
        contents: vec![]
      });
    }

    if split[1].trim().starts_with("no") {
      continue;
    }

    for part in split[1].trim().split(",") {
      let count: u8 = part.trim().split(" ").next().unwrap().trim().parse().unwrap();
      let other_name = part[2..].trim().trim_end_matches("bags.").trim_end_matches("bags").trim_end_matches("bag").trim_end_matches("bag.").trim();
    
      to_return.get_mut(name).unwrap().contents.push(BagContents {
        amount: count,
        bag: other_name.to_owned()
      });
    }
  }

  to_return
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &HashMap<String, Bag>) -> u32 {
  let mut to_return = 0;

  for (_, bag) in input {
    to_return += count(&bag, input, "shiny gold", 0);
  }

  to_return
}

fn count(bag: &Bag, bags: &HashMap<String, Bag>, target: &str, min_count: u32) -> u32 {
  for contents in bag.contents.iter() {
    if contents.bag == target && contents.amount as u32 >= min_count {
      return 1;
    }

    if let Some(other_bag) = bags.get(&contents.bag) {
      let child = count(other_bag, bags, target, min_count);
      if child > 0 {
        return 1;
      }
    }
  }

  return 0;
}


#[aoc(day7, part2)]
pub fn solve_part2(input: &HashMap<String, Bag>) -> u32 {
  return count_children(input.get("shiny gold").unwrap(), input) - 1
}

fn count_children(bag: &Bag, bags: &HashMap<String, Bag>) -> u32 {
  let mut to_return = 1;

  for contents in bag.contents.iter() {
    if let Some(other_bag) = bags.get(&contents.bag) {
      to_return += contents.amount as u32 * count_children(&other_bag, bags);
    }
  }

  to_return
}