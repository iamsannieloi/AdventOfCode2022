use crate::common;

pub fn solution() -> i32 {
  let lines = common::lines_from_file("../inputs/day04.txt").expect("did the file get read?");

  //let result = lines.iter().map(|x| is_contain_full_overlap_section(x)).collect::<Vec<bool>>();
  let result = lines.iter().map(|x| is_contain_any_overlap_section(x)).collect::<Vec<bool>>();
  let filter_result: Vec<&bool> = result.iter().filter(|&y| *y).collect::<Vec<&bool>>();

  return filter_result.len() as i32;
}

fn is_contain_full_overlap_section(sections_info: &String) -> bool {
  // println!("STRING: {:?}", sections_info);
  let split : Vec<&str> = sections_info.split(",").collect();
  let elf_one: Vec<i32> = split[0].split("-").collect::<Vec<&str>>().iter().map(|x| x.parse().unwrap()).collect();
  let elf_two: Vec<i32> = split[1].split("-").collect::<Vec<&str>>().iter().map(|x| x.parse().unwrap()).collect();
  //println!("SPLIT: {:?}", elf_one);
  if elf_one[0] == elf_two[0] || elf_one[1] == elf_two[1] {
    return true;
  } else if elf_one[0] < elf_two[0] {
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
  } else {
    return false;
  }
}

fn is_contain_any_overlap_section(sections_info: &String) -> bool {
  let split : Vec<&str> = sections_info.split(",").collect();
  let elf_one: Vec<i32> = split[0].split("-").collect::<Vec<&str>>().iter().map(|x| x.parse().unwrap()).collect();
  let elf_two: Vec<i32> = split[1].split("-").collect::<Vec<&str>>().iter().map(|x| x.parse().unwrap()).collect();
  if elf_one[0] <= elf_two[0] && elf_two[0] <= elf_one[1] {
    return true;
  }
  else if elf_two[0] <= elf_one[0] && elf_one[0] <= elf_two[1] {
    return true;
  } else {
    return false;
  }
}