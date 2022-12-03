use std::collections::HashMap;
use std::env;
use crate::common;


pub fn solution() -> i32 {
  let mut total = 0;
  let mut total_part2 = 0;
  let lines = common::lines_from_file("../inputs/day02.txt").expect("did the file get read?");
  for line in lines {
    total += get_winner(&line);
    total_part2 += get_winner_part2(&line);
  }

  return total_part2;
}

fn get_winner(moves: &str) -> i32 {

  let rock_paper_scissors_map :HashMap<&str, i32> = HashMap::from([
    ("A X", (3 + 1)),
    ("B Y", (3 + 2)),
    ("C Z", (3 + 3)),
    ("A Y", (6 + 2)),
    ("A Z", (0 + 3)),
    ("B X", (0 + 1)),
    ("B Z", (6 + 3)),
    ("C X", (6 + 1)),
    ("C Y", (0 + 2)),
  ]);
  let result = rock_paper_scissors_map.get(moves).unwrap();
  //println!("RESULT:{:?}", result);
  return *result;
}

fn get_winner_part2(moves: &str) -> i32 {

  let rock_paper_scissors_map :HashMap<&str, i32> = HashMap::from([
    ("A X", (0 + 3)),
    ("B X", (0 + 1)),
    ("C X", (0 + 2)),
    ("A Y", (3 + 1)),
    ("B Y", (3 + 2)),
    ("C Y", (3 + 3)),
    ("A Z", (6 + 2)),
    ("B Z", (6 + 3)),
    ("C Z", (6 + 1)),
  ]);
  let result = rock_paper_scissors_map.get(moves).unwrap();
  //println!("RESULT:{:?}", result);
  return *result;
}
