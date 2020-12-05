use aoc_runner_derive::{aoc_generator, aoc};
use std::collections::HashMap;

pub struct Passport {
  birth_year: Option<String>,
  issue_year: Option<String>,
  expiration_year: Option<String>,
  height: Option<String>,
  hair_color: Option<String>,
  eye_color: Option<String>,
  id: Option<String>
}

impl Passport {
  fn from_hashmap(map: HashMap<&str, String>) -> Self {
    Passport {
      birth_year: map.get("birth_year").map(|i| i.to_string()),
      issue_year: map.get("issue_year").map(|i| i.to_string()),
      expiration_year: map.get("expiration_year").map(|i| i.to_string()),
      height: map.get("height").map(|i| i.to_string()),
      hair_color: map.get("hair_color").map(|i| i.to_string()),
      eye_color: map.get("eye_color").map(|i| i.to_string()),
      id: map.get("id").map(|i| i.to_string())
    }
  }

  fn is_valid(&self) -> bool {
    self.birth_year.is_some() && 
    self.issue_year.is_some() &&
    self.expiration_year.is_some() &&
    self.height.is_some() &&
    self.hair_color.is_some() &&
    self.eye_color.is_some() &&
    self.id.is_some()
  }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
  let mut passports = vec![];
  let split = input.split("\n\n").map(|line| str::replace(line, "\n", " ")).collect::<Vec<String>>();

  for line in split {
    let details = line.split(" ");
    let mut field_map = HashMap::new();

    for detail in details {
      let parts = detail.split(":").collect::<Vec<&str>>();
      let key = parts[0];
      let value = parts[1].to_owned();

      match key {
        "byr" => field_map.insert("birth_year", value),
        "iyr" => field_map.insert("issue_year", value),
        "eyr" => field_map.insert("expiration_year", value),
        "hgt" => field_map.insert("height", value),
        "hcl" => field_map.insert("hair_color", value),
        "ecl" => field_map.insert("eye_color", value),
        "pid" => field_map.insert("id", value),
        _ => None
      };
    }

    passports.push(Passport::from_hashmap(field_map))
  }

  passports
}


#[aoc(day4, part1)]
pub fn solve_part1(passports: &Vec<Passport>) -> usize {
  passports.iter().filter(|passport| passport.is_valid()).count()
}

#[aoc(day4, part2)]
pub fn solve_part2(passports: &Vec<Passport>) -> usize {
  fn filter(passport: &Passport) -> std::io::Result<bool> {
    let mut to_return = 1;

    if let Some(birth_year) = &passport.birth_year {
      let birth_year: u16 = birth_year.parse().unwrap();
      to_return *= (birth_year >= 1920 && birth_year <= 2002) as u32;
    }

    if let Some(issue_year) = &passport.issue_year {
      let issue_year: u16 = issue_year.parse().unwrap();
      to_return *= (issue_year >= 2010 && issue_year <= 2020) as u32;
    }

    if let Some(expiration_year) = &passport.expiration_year {
      let expiration_year: u16 = expiration_year.parse().unwrap();
      to_return *= (expiration_year >= 2020 && expiration_year <= 2030) as u32;
    }

    if let Some(height) = &passport.height {
      if height.contains("cm") {
        let height: u16 = str::replace(&height, "cm", "").parse().unwrap();
        to_return *= (height >= 150 && height <= 193) as u32;
      } else {
        let height: u16 = str::replace(&height, "in", "").parse().unwrap();
        to_return *= (height >= 59 && height <= 76) as u32;
      }
    }

    if let Some(hair_color) = &passport.hair_color {
      if hair_color.starts_with('#') && hair_color.len() == 7 {
        to_return *= (u32::from_str_radix(&hair_color.trim_start_matches("#"), 16).is_ok()) as u32;
      } else {
        to_return = 0
      }
    }

    if let Some(eye_color) = &passport.eye_color {
      to_return *= match eye_color.as_ref() {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => 1,
        _ => 0
      }
    }

    if let Some(id) = &passport.id {
      to_return *= (id.chars().all(char::is_numeric) && id.len() == 9) as u32
    }

    Ok(to_return != 0)
  }

  passports.iter().filter(|passport| passport.is_valid() && filter(*passport).unwrap()).count()
}