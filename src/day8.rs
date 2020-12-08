use aoc_runner_derive::{aoc_generator, aoc};
use num::integer::Integer;

#[derive(Debug, Clone)]
pub enum Instruction {
  Acc(i16),
  Jmp(i16),
  Nop(i16)
}

pub struct CPU {
  accumulator: i16,
  counter: u16,
  bitmasks: Vec<u64>
}

impl CPU {
  fn new(alloc: usize) -> Self {
    Self {
      accumulator: 0,
      counter: 0,
      bitmasks: (0..(alloc.div_ceil(&64))).map(|_| 0).collect()
    }
  }

  fn run(&mut self, instruction: &Instruction) {
    match instruction {
      Instruction::Acc(operand) => {
        self.accumulator += operand
      },

      Instruction::Jmp(operand) => {
        self.counter = (self.counter as i16 + *operand) as u16
      },
      
      Instruction::Nop(_) => ()
    }
  }
}

impl Instruction {
  fn from_string<S: Into<String>>(string: S) -> Self {
    let string = string.into();
    let items = string.split(" ").collect::<Vec<&str>>();
    let op = items[0];
    let operand: i16 = items[1].parse().unwrap();

    match op {
      "acc" => {
        Instruction::Acc(operand)
      },
      "jmp" => {
        Instruction::Jmp(operand)
      },
      "nop" => {
        Instruction::Nop(operand)
      }
      _ => { unreachable!() }
    }
  }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
   input
    .lines()
    .map(|line| Instruction::from_string(line)).collect::<Vec<Instruction>>()
}


#[aoc(day8, part1)]
pub fn solve_part1(input: &Vec<Instruction>) -> i16 {
  let mut cpu = CPU::new(input.len());

  loop {
    let counter = cpu.counter;
    let mut bitmask = cpu.bitmasks[(counter / 64) as usize];
    let instruction = &input[counter as usize];
    let flag: u64 = 1 << counter % 64;

    if bitmask & flag > 0 {
      return cpu.accumulator;
    }

    cpu.run(&instruction);

    bitmask |= flag;
    cpu.bitmasks[(counter / 64) as usize] = bitmask;

    if counter == cpu.counter {
      cpu.counter += 1;
    }
  }
}

fn completes(input: &Vec<Instruction>) -> (bool, i16) {
  let mut cpu = CPU::new(input.len());

  while (cpu.counter as usize) < input.len() {
    let counter = cpu.counter;
    let mut bitmask = cpu.bitmasks[(counter / 64) as usize];
    let instruction = &input[counter as usize];
    let flag: u64 = 1 << counter % 64;

    if bitmask & flag > 0 {
      return (false, cpu.accumulator);
    }

    cpu.run(&instruction);

    bitmask |= flag;
    cpu.bitmasks[(counter / 64) as usize] = bitmask;

    if counter == cpu.counter {
      cpu.counter += 1;
    }
  }

  (true, cpu.accumulator)
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Vec<Instruction>) -> i16 {
  let mut cpu = CPU::new(input.len());
  let input = input.clone();
  let mut instruction_cache = vec![];

  loop {
    let counter = cpu.counter;
    let mut bitmask = cpu.bitmasks[(counter / 64) as usize];
    let instruction = &input[counter as usize];
    let flag: u64 = 1 << counter % 64;

    if bitmask & flag > 0 {
      for previous in instruction_cache.iter().rev() {
        let mut cloned = input.clone();
        let index = *previous as usize;
        cloned[index] = match cloned[index] {
          Instruction::Jmp(operand) => Instruction::Nop(operand),
          Instruction::Nop(operand) => Instruction::Jmp(operand),
          _ => cloned[index].clone()
        };

        let result = completes(&cloned);
        if result.0 {
          return result.1
        }
      }
    }

    instruction_cache.push(counter);

    cpu.run(&instruction);

    bitmask |= flag;
    cpu.bitmasks[(counter / 64) as usize] = bitmask;

    if counter == cpu.counter {
      cpu.counter += 1;
    }
  }
}