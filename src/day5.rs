use aoc_runner_derive::{aoc_generator, aoc};
use num::integer::Integer;

type BoardingPass = u16;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<String> {
  input.lines()
    .map(|s| s.to_owned())
    .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Vec<String>) -> BoardingPass {
  let mut passes = parse(input);
  passes.sort_by(|a, b| b.cmp(&a));
  passes[0]
}

#[aoc(day5, part1, binary)]
pub fn solve_part1_binary(input: &Vec<String>) -> BoardingPass {
  let mut highest: BoardingPass = 0;

  for line in input {
    let chars: Vec<char> = line.chars().collect();
    let mut row: String = chars[0..7].iter().collect();
    let mut column: String = chars[7..].iter().collect();

    row = str::replace(&row, "F", "0");
    row = str::replace(&row, "B", "1");
    column = str::replace(&column, "L", "0");
    column = str::replace(&column, "R", "1");

    let row: u16 = u16::from_str_radix(&row, 2).unwrap_or(0);
    let column: u16 = u16::from_str_radix(&column, 2).unwrap_or(0);
    let pass: BoardingPass = row * 8 + column;

    if pass > highest {
      highest = pass;
    }
  }

  highest
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Vec<String>) -> BoardingPass {
  let mut passes = parse(input);
  passes.sort_by(|a, b| b.cmp(&a));

  let mut previous_pass: Option<BoardingPass> = None;
  for pass in passes {
    if previous_pass.is_none() {
      previous_pass = Some(pass);
      continue;
    }

    if let Some(previous) = &previous_pass {
      if previous - pass > 1 {
        return previous - 1
      }
    }

    previous_pass = Some(pass);
  }

  unreachable!()
}


fn parse(input: &Vec<String>) -> Vec<BoardingPass> {
  let mut passes = vec![];

  for line in input {
    let mut chars: Vec<char> = line.chars().collect();
    let mut row = 0;
    let mut column = 0;

    let mut max: u8 = 127;
    let mut min = 0;
    let mut previous = 'L';
    while chars.len() > 0 {
      let mut current = chars[0];

      if (previous == 'F' || previous == 'B') && (current == 'R' || current == 'L') {
        match previous {
          'F' => {
            row = min
          },
          'B' => {
            row = max
          },
          _ => ()
        }

        min = 0;
        max = 7;
        previous = 'L';
        
        continue;
      }

      current = chars.remove(0);

      match current {
        'F' | 'L' => {
          max = max - (max - min).div_ceil(&2);
        },
        'B' | 'R' => {
          min = ((max - min).div_ceil(&2)) + min;
        }
        _ => ()
      }

      previous = current;
    }

    match previous {
      'L' => {
        column = min
      },
      'R' => {
        column = max
      },
      _ => ()
    }

    passes.push((row as u16 * 8) + column as u16);
  }

  passes
}