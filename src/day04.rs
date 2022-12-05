use std::env;
use crate::common;

pub fn solution() -> i32 {
  //let lines = common::lines_from_file("../inputs/day03.txt").expect("did the file get read?");
  let lines = common::lines_from_file("../inputs/day04.txt").expect("did the file get read?");

  let result = lines.iter().map(|x| is_contain_section(x)).collect::<Vec<bool>>();
  let filter_result: Vec<&bool> = result.iter().filter(|&y| *y).collect::<Vec<&bool>>();

  return filter_result.len() as i32;
}

fn is_contain_section(sections_info: &String) -> bool {
  // println!("STRING: {:?}", sections_info);
  let split : Vec<&str> = sections_info.split(",").collect();
  let elf_one: Vec<&str> = split[0].split("-").collect();
  let elf_two: Vec<&str> = split[1].split("-").collect();
  //println!("SPLIT: {:?}", elf_one);
  if elf_one[0] < elf_two[0] {
    if elf_one[1] >= elf_two[1] {
      return true;
    } else {
      return false;
    }
  }
  else if elf_one[0] > elf_two[0] {
    if elf_one[1] <= elf_two[1] {
      return true;
    } else {
      return false;
    }
  } else if elf_one[0] == elf_two[0] {
    return true;
  } else {
    println!("HERE");
    return false;
  }
}