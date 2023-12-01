use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

pub mod sol2;

pub fn main() {

    match File::open("input.txt") {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut numbers:Vec<usize> = vec![]; 
            for line_result in reader.lines() {
                match line_result {
                    Ok(line) => {
                        let (mut left, mut right) = (0, line.len().saturating_sub(1));
                        let mut first_num = None;
                        let mut second_num = None;

                        while left <= right {
                            

                            if first_num.is_none() && line.chars().nth(left).unwrap().is_numeric() {
                                first_num = line.chars().nth(left).map(|c| c.to_string());
                            }


                            if second_num.is_none() && line.chars().nth(right).unwrap().is_numeric() {
                                let half = line.len() as f32 / 2.0;
                                let condition = left  as f32 - half;
                                if  condition > 0.0 {
                                    break;
                                }

                                second_num = line.chars().nth(right).map(|c| c.to_string());
                            }

                            if first_num.is_some() && second_num.is_some() {
                                break;
                            }

                            if first_num.is_none() { left += 1; }
                            if second_num.is_none() { right = right.saturating_sub(1); }
                        }

                        let concatenated_results = format!("{}{}", first_num.unwrap_or_default(), second_num.unwrap_or_default());

                        match concatenated_results.parse::<usize>(){
                            Ok(number) => numbers.push(number),
                            Err(e) => {
                                
                                println!("line: {:?}{:?}{:?}", line,right,right);    
                                println!("Failed to parse into a number, {}", e)
                            },
                        }

                        println!("Concatenated Numbers: {}", concatenated_results);
                    },
                    Err(e) => println!("Error reading line: {}", e),
                }
            }
            println!("Total Number: {}", numbers.iter().sum::<usize>());
        },
        Err(e) => println!("Error opening file: {}", e),
    }
}
