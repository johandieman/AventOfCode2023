use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

pub fn main() {
    let scores = HashMap::from([
        ("one", 1), ("two", 2), ("three", 3), ("four", 4), 
        ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), 
        ("nine", 9),
    ]);

    match File::open("input.txt") {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut numbers: Vec<usize> = vec![];

            for line_result in reader.lines() {
                match line_result {
                    Ok(line) => {
                        let mut first_num = None;
                        let mut last_num = None;
                        let mut left = 0;
                        let mut right = line.len();

                        while left < right {
                            for (word, &num) in &scores {
                                if first_num.is_none() && line[left..].starts_with(word) {
                                    first_num = Some(num.to_string());
                                }
                                if last_num.is_none() && line[..right].ends_with(word) {
                                    last_num = Some(num.to_string());
                                }
                            }

                            if first_num.is_none() && line.chars().nth(left).unwrap().is_numeric() {
                                first_num = line.chars().nth(left).map(|c| c.to_string());
                            }
                            if last_num.is_none() && line.chars().nth(right - 1).unwrap().is_numeric() {
                                last_num = line.chars().nth(right - 1).map(|c| c.to_string());
                            }

                            if first_num.is_some() && last_num.is_some() {
                                break;
                            }

                            if first_num.is_none() {
                                left += 1;
                            }
                            if last_num.is_none() && right > 0 {
                                right = right.saturating_sub(1); 
                            }
                        }

                        let concatenated_results = format!(
                            "{}{}", 
                            first_num.unwrap_or_default(), 
                            last_num.unwrap_or_default()
                        );

                        println!("Concatenated Numbers for this line: {}", concatenated_results); 

                        match concatenated_results.parse::<usize>() {
                            Ok(number) => {
                                numbers.push(number);

                            },
                            Err(_) => println!("Failed to parse '{}' into a number", concatenated_results),
                        }
                    },
                    Err(e) => println!("Error reading line: {}", e),
                }
            }
            println!("Total Number (Sum of all lines): {}", numbers.iter().sum::<usize>()); 
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
