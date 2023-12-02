use std::env::var;
use std::io::{BufRead, BufReader};
use std::fs::File;

struct Info {
    game: i32,
    green: i32,
    blue: i32,
    red: i32,
}

impl Info {
    fn new(game: i32, green: i32, blue: i32, red: i32) -> Info {
        Info { game, green, blue, red }
    }
    fn total(&self) -> i32 {
        self.green * self.blue * self.red
    }
}

pub fn main(){
    let mut sum = 0;
    match File::open("input.txt") {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line_result in reader.lines() {
                match line_result {
                    Ok(line)=>{
                        let mut initial = line.split(':');
                        let game = initial.next().unwrap().split_whitespace().last();
                        let game_num = convert_option_str_to_i32(game).unwrap();
                        let mut vars = Info::new(game_num,0,0,0);
                        let sets = initial.next().unwrap().split(';');
                        for set in sets {
                            let trimmed: String = set.chars().filter(|c| !c.is_whitespace()).collect();
                            let items = trimmed.split(',');
                            for item in items {
                                println!("item: {}", item);                          
                                if item.contains("blue"){
                                    let str_num = item.replace("blue","");
                                    let num = convert_string_to_i32(str_num).unwrap();
                                    if num > vars.blue {
                                        vars.blue = num;
                                    }

                                } else if item.contains("green") {
                                    let str_num = item.replace("green","");
                                    let num = convert_string_to_i32(str_num).unwrap();

                                    if num > vars.green {
                                        vars.green = num;

                                    }
                                    // vars.green = num;
                                } else if item.contains("red") {
                                    let str_num = item.replace("red","");
                                    let num = convert_string_to_i32(str_num).unwrap();

                                    if num > vars.red {
                                        vars.red = num;

                                    }
                                    // vars.red = convert_string_to_i32(num).unwrap();
                                } else {
                                    continue;
                                }
                            }
                        }
                        
                        sum += vars.total();
                    },
                    Err(e) => println!("Error reading line: {}", e),
                }
            }
            println!("Total: {}", sum);



        },
        Err(e) => println!("Error reading line: {}", e),
    }
}


fn convert_string_to_i32(option_str: String) -> Result<i32, String> {
    match option_str.parse::<i32>() {
        Ok(parsed_int) => Ok(parsed_int),
        Err(_) => Err("String could not be parsed as i32".to_string()),
    }
}

fn convert_option_str_to_i32(option_string: Option<&str>) -> Result<i32, String> {
    match option_string {
        Some(value) => {
            match value.parse::<i32>() {
                Ok(parsed_int) => Ok(parsed_int),
                Err(_) => Err("String could not be parsed as i32".to_string()),
            }
        },
        None => Err("No value provided".to_string()),
    }
}