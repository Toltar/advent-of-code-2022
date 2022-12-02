use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};

fn get_total_calories_by_elf(lines: Vec<&str>) -> Vec<i32> {
    let last_value = *lines.last().unwrap();
    let line_iterator = lines.into_iter();
    let mut current_total = 0;
    let mut total_calories_by_elf = Vec::new();
    line_iterator.for_each(|line| {
        if line.trim().is_empty() {
            total_calories_by_elf.push(current_total.clone());
            current_total = 0;
        } else {
            current_total += line.parse::<i32>().unwrap();
            if line == last_value {
                total_calories_by_elf.push(current_total)
            }
        }
    });

    return total_calories_by_elf;
}

fn find_top_three_total(total_by_elf: Vec<i32>) -> i32 {
    let mut vector_clone = total_by_elf.clone();
    let len = vector_clone.len();
    vector_clone.sort();
    let total = vector_clone[len - 1] + vector_clone[len - 2] + vector_clone[len - 3];
    return total;
}

fn main() {
    let file_path: &str = "./input.txt";
    let read_to_string_result =
        read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<_> = read_to_string_result.split("\n").collect();

    let total_calories_by_elf = get_total_calories_by_elf(lines);
    let max_calories = total_calories_by_elf.iter().max().unwrap();

    println!("Max calories {}", max_calories);
    println!(
        "Total of top 3 elves {}",
        find_top_three_total(total_calories_by_elf)
    );

    // println!("Output {}", items_by_elf.next().unwrap()[0])
}
