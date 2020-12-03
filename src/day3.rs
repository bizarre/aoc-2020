use aoc_runner_derive::{aoc_generator, aoc};
use std::io::{Error, ErrorKind};

#[derive(Clone)]
pub struct Grid {
  lines: Vec<String>,
  x: u16,
  y: u16
}

impl Grid {
  fn new<S: Into<String>>(input: S) -> Self {
    Grid {
      lines: input.into().lines().map(|l| l.to_owned()).collect(),
      x: 0,
      y: 0
    }
  }

  fn get_next(&mut self, skip_x: u16, skip_y: u16) -> std::io::Result<char> {
    let y = self.y + skip_y;

    if y as usize >= self.lines.len() {
      return Err(Error::new(ErrorKind::Other, "Bottom of mountain!"));
    }

    let line = &self.lines[y as usize];
    let mut x = self.x + skip_x;

    if x as usize >= line.len() {
      x = x - (line.len() as u16);
    }

    self.x = x;
    self.y = y;

    Ok(line.chars().collect::<Vec<char>>()[x as usize])
  }
}


#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Grid {
  Grid::new(input)
}

const TREE: char = '#';

#[aoc(day3, part1)]
pub fn solve_part1(grid: &Grid) -> u32 {
  let mut grid = grid.clone();
  let mut trees = 0;

  while let Ok(item) = grid.get_next(3, 1) {
    trees += (item == TREE) as u32
  }

  trees
}

#[aoc(day3, part2)]
pub fn solve_part2(grid: &Grid) -> u32 {
  let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
  let mut trees = 1;

  for slope in slopes {
    let mut grid = grid.clone();
    let mut local_trees = 0;
    
    while let Ok(item) = grid.get_next(slope.0, slope.1) {
      local_trees += (item == TREE) as u32;
    }

    trees *= local_trees
  }

  trees
}
