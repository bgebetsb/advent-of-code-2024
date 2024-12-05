use std::fs::read_to_string;
use std::io;
use utils::*;

fn check_valid(rules: &Vec<Vec<i32>>, input: &Vec<i32>) -> bool {
    for i in 0..input.len() {
        for rule in rules {
            if rule[0] == input[i] {
                for j in 0..i {
                    if rule[1] == input[j] {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn check_valid_swap(rules: &Vec<Vec<i32>>, input: &mut Vec<i32>) -> bool {
    let mut sorted = true;
    for i in 0..input.len() {
        for rule in rules {
            if rule[0] == input[i] {
                for j in 0..i {
                    if rule[1] == input[j] {
                        let tmp = input[i];
                        input[i] = input[j];
                        input[j] = tmp;
                        sorted = false;
                    }
                }
            }
        }
    }
    sorted
}

fn main() -> Result<(), io::Error> {
    let lines = read_to_string("input.txt")?.get_lines();

    let rules = lines.numbers_in_line(b'|');
    let mut items = lines.numbers_in_line(b',');

    let mut sum = 0;
    let mut sum_part_2 = 0;
    for item in &mut items {
        if check_valid(&rules, &item) {
            sum += item[item.len() / 2];
        } else {
            while !check_valid_swap(&rules, item) {}
            sum_part_2 += item[item.len() / 2];
        }
    }

    println!("{}", sum);
    println!("{}", sum_part_2);

    Ok(())
}
