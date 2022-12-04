use std::env;
use crate::common;

pub fn solution() -> i32 {
  let lines = common::lines_from_file("../inputs/day03.txt").expect("did the file get read?");

  // part 1
  //let result : i32 = lines.iter().map(|line| find_same_item_type(line)).collect::<Vec<i32>>().iter().sum();

  // part2
  let mut count : usize = 0;
  let mut rucksack_group : [&String; 3] = [&"".to_string(),&"".to_string(),&"".to_string()];
  let mut priority_list= Vec::new();
  for i in 0..lines.len(){
    if count == 2 {
      rucksack_group[count] = &lines[i];
      let priority : i32 = find_same_item_group(rucksack_group);
      priority_list.push(priority);
      count = 0;
    } else {
      rucksack_group[count] = &lines[i];
      count += 1;
    }
  }
  let result : i32 = priority_list.iter().sum();
  //println!("RESULT {:?}", result);
  return result;
}

fn find_same_item_type(rucksack: &String) -> i32 {
  let rucksack_len = rucksack.len();
  let half_len = rucksack_len / 2;
  let char_vec: Vec<char> = rucksack[..half_len].chars().collect();

  let same_item_type : Vec<&char> = char_vec.iter().filter(|&x| rucksack[half_len..rucksack_len].contains(*x)).collect();
  let priority : i32 = calculate_item_priority(same_item_type[0]);
  return priority;
}

fn find_same_item_group(rucksack_group: [&String; 3])  -> i32 {
  let char_vec: Vec<char> = rucksack_group[0].chars().collect();
  let compare_first_two: Vec<&char> = char_vec.iter().filter(|&x| rucksack_group[1].contains(*x)).collect();
  let compare_with_last: Vec<&&char> = compare_first_two.iter().filter(|&x| rucksack_group[2].contains(**x)).collect();

  let priority : i32 = calculate_item_priority(compare_with_last[0]);
  return priority;
}

fn calculate_item_priority(item: &char) -> i32 {

  let item_priority: i32 = *item as i32 % 32;
  if item.is_uppercase(){
    return 26 + item_priority;
  } 
  return item_priority;
}
