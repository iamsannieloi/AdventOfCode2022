use std::env;
use crate::common;

pub fn solution() -> i32 {
  let mut calories = Vec::new();
  let mut current = 0;
  let lines = common::lines_from_file("../inputs/day1.txt").expect("did the file get read?");
  for line in lines {
    if line.is_empty() {
      calories.push(current);
      current = 0;
    } else {
      let curnum: i32 = line.parse().unwrap();
      current += curnum;
    }
  }
  calories.sort();
  let veclen = calories.len();
  let total: i32 = calories[(veclen-3)..veclen].iter().sum();

  return total;
}
