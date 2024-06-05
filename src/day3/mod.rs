use std::{collections::HashMap, io::{BufReader,BufRead}, fs::File};
use std::mem;


#[derive(Debug)]
struct LineInfo {
    number_positions: HashMap<i32, Vec<usize>>,
    activator_positions: HashMap<char, Vec<usize>>,
}

impl LineInfo {
    fn new() -> LineInfo {
        LineInfo {
            number_positions: HashMap::new(),
            activator_positions: HashMap::new(),
        }
    }
}

pub fn main() {
    match File::open("input.txt") {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut lines:Vec<LineInfo> = vec![];
            for line_result in reader.lines() {
                match line_result {
                    Ok(line)=>{
                        let mut line_info = LineInfo::new();

                        let mut full_num_storage = String::new();
                        let mut register_storage = String::new();
                        let mut counter = 0;

                        for (index, char) in line.chars().enumerate() {

                            if char.is_digit(10) {
                                register_storage.push(char);
                            } else {

                                match convert_to_i32(&register_storage) {
                                    Ok(_)=>{
                                        println!("char {}, store1 {}, num{}",char,register_storage,full_num_storage);

                                        full_num_storage = mem::replace(&mut register_storage, String::new());

                                        // register_storage.clear();
                                    },
                                    Err(_)=>{
                                        println!("char {}, store2 {}, num{}",char,register_storage,full_num_storage);

                                        match convert_to_i32(&full_num_storage){
                                            Ok(num)=>{
                                                println!("num parsed{}", num);
                                                line_info.number_positions.insert(num, vec![index-register_storage.len(), index-1]);
                                            },
                                            Err(e)=>{
                                                // if char == '.' { continue };
                                                println!("char {}, store3 {}, num{}",char,register_storage,full_num_storage);


                                                match line_info.activator_positions.entry(char) {
                                                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                                                        entry.get_mut().push(index);
                                                    },
                                                    std::collections::hash_map::Entry::Vacant(entry) => {

                                                        entry.insert(vec![index]);
                                                    }
                                                }
                                                
                                            }
                                        }

                                        full_num_storage.clear();

                                        
                                    }
                                }
                            }

                        }

                        lines.push(line_info)



                    },
                    Err(e)=>println!("{}",e)
                }
            }



            for line in lines {

                println!("{:?}", line);
            }
        },
        Err(e)=>println!("{}",e)
    }
}

fn convert_to_i32<T: ParseToInt>(input: T) -> Result<i32, String> {
    input.parse_to_i32()
}

trait ParseToInt {
    fn parse_to_i32(&self) -> Result<i32, String>;
}

impl ParseToInt for String {
    fn parse_to_i32(&self) -> Result<i32, String> {
        self.parse::<i32>().map_err(|_| "Failed to parse str".to_string())
    }
}

impl ParseToInt for &String {
    fn parse_to_i32(&self) -> Result<i32, String> {
        self.parse::<i32>().map_err(|_| "Failed to parse str".to_string())
    }
}

impl ParseToInt for Option<&String> {
    fn parse_to_i32(&self) -> Result<i32, String> {
        match self {
            Some(value) => value.parse::<i32>().map_err(|_| "Failed to parse str".to_string()),
            None => Err("No value provided".to_string()),
        }
    }
}

impl ParseToInt for Option<String> {
    fn parse_to_i32(&self) -> Result<i32, String> {
        match self {
            Some(value) => value.parse::<i32>().map_err(|_| "Failed to parse str".to_string()),
            None => Err("No value provided".to_string()),
        }
    }
}

impl ParseToInt for Option<char> {
    fn parse_to_i32(&self) -> Result<i32, String> {
        match self {
            Some(value) => value.to_string().parse::<i32>().map_err(|_| "Failed to parse char".to_string()),
            None => Err("No value provided".to_string()),
        }
    }
}