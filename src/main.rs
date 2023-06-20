use regex::Regex;
use std::fmt;

// Define all "states" and their matching functions
// Possibly the worst possible recursive decent praser lol
// Consider this v0.0.0.0.0.0.0.0.0.1 of maybe something ill build seriously lol
// Obviously cannot handle left recursive or left factored
#[derive(Debug)]
enum States {
    START,
    TYPE,
    ARGS,
    DECL,
}

impl fmt::Display for States {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            States::START => write!(f, "START"),
            States::TYPE => write!(f, "TYPE"),
            States::ARGS => write!(f, "ARGS"),
            States::DECL => write!(f, "DECL"),
        }
    }
}

struct MatchStacks {
    START: Vec<Vec<String>>,
    TYPE: Vec<Vec<String>>,
    ARGS: Vec<Vec<String>>,
    DECL: Vec<Vec<String>>,
}

//Defining the grammer and match rules :
impl Default for MatchStacks {
    fn default() -> Self {
        MatchStacks {
            START: vec![vec![
                States::TYPE.to_string(),
                "([a-zA-Z_][a-zA-Z0-9_]*)".to_string(),
                r"(\()".to_string(),
                States::ARGS.to_string(),
                r"(\))".to_string(),
                r"(\{)".to_string(),
                r"([^\}]*)".to_string(),
                r"(\})".to_string(),
            ]],
            TYPE: vec![
                vec!["(int )".to_string()],
                vec!["(void )".to_string()],
                vec!["(T )".to_string()],
            ],
            DECL: vec![vec![
                States::TYPE.to_string(),
                "([a-zA-Z_][a-zA-Z0-9_]*)".to_string(),
            ]],
            ARGS: vec![
                vec![
                    States::DECL.to_string(),
                    "(,)".to_string(),
                    States::ARGS.to_string(),
                ],
                vec![States::DECL.to_string()],
                vec![],
            ],
        }
    }
}

fn regex_matches<'a>(regex_pat: &str, offset: usize, code: &'a str) -> (bool, &'a str) {
    let new_regex_matcher = Regex::new(regex_pat).unwrap();
    match new_regex_matcher.captures_at(code, offset) {
        Some(matches) => return (true, matches.get(0).unwrap().as_str().clone()),
        None => return (false, ""),
    };
}

fn recursive_decent_parser(
    match_stack: Vec<Vec<String>>,
    offset: &mut usize,
    code: &str,
) -> (bool, usize) {
    //We need to clone and replicate the input buffer into the next recursive call so that we ca
    //return back to the original posistion in the stack!
    let inital_offset = offset.clone();
    let mut largest_offset = 0 as usize;
    let mut match_position: Vec<i32> = Vec::new();
    for _ in match_stack.iter().enumerate() {
        match_position.push(0)
    }
    let mut final_response = false;
    let mut matched_stack: i32 = -1 as i32;
    'outer: for (i, v) in match_stack.iter().enumerate() {
        let mut stack_reponse = false;
        *offset = inital_offset;

        //Empty case :
        if v.len() == 0 {
            stack_reponse = true;
        }
        for (_, j) in v.iter().enumerate() {
            // [TODO] Should be able to write this match condition withotu all this repetition
            // [TODO] Match string to type and respectively funciton call
            match j.as_str() {
                //States need to go to recursive searches
                "TYPE" => {
                    println!("TYPE");
                    let response = recursive_decent_parser(
                        MatchStacks::default().TYPE,
                        &mut offset.clone(),
                        code,
                    );
                    println!("{:?}", response);
                    if stack_reponse == false || response.0 == true {
                        stack_reponse = response.0;
                        *offset = response.1;
                    } else if response.0 == false {
                        continue 'outer;
                    }
                }
                "DECL" => {
                    println!("DECL");
                    let response = recursive_decent_parser(
                        MatchStacks::default().DECL,
                        &mut offset.clone(),
                        code,
                    );
                    println!("{:?}", response);
                    if stack_reponse == false || response.0 == true {
                        stack_reponse = response.0;
                        *offset = response.1;
                    } else if response.0 == false {
                        continue 'outer;
                    }
                }
                "ARGS" => {
                    println!("ARGS");
                    let response = recursive_decent_parser(
                        MatchStacks::default().ARGS,
                        &mut offset.clone(),
                        code,
                    );
                    println!("{:?}", response);
                    if stack_reponse == false || response.0 == true {
                        stack_reponse = response.0;
                        *offset = response.1;
                    } else if response.0 == false {
                        continue 'outer;
                    }
                }
                "START" => {
                    println!("START");
                    let response = recursive_decent_parser(
                        MatchStacks::default().START,
                        &mut offset.clone(),
                        code,
                    );
                    println!("{:?}", response);
                    if stack_reponse == false || response.0 == true {
                        stack_reponse = response.0;
                        *offset = response.1;
                    } else if response.0 == false {
                        continue 'outer;
                    }
                }
                _ => {
                    //Greedy match using regex functions
                    let response = regex_matches(&j, offset.clone(), code);
                    if response.0 == false {
                        *offset = inital_offset;
                        continue 'outer;
                    } else {
                        stack_reponse = true;
                        *offset += response.1.len();
                        println!("{:?}", response.1);
                        continue;
                    }
                }
            }
        }
        if stack_reponse == true {
            if *offset > largest_offset {
                largest_offset = *offset;
            }
            final_response = true;
            if matched_stack == -1 {
                matched_stack = i as i32;
            } else if match_stack.get(matched_stack as usize).unwrap().len() < v.len() {
                matched_stack = i as i32;
            }
        }
    }
    if final_response == true {
        return (true, largest_offset);
    }
    return (false, inital_offset);
}

fn main() {
    println!("Parsing code...");
    let mut offset: usize = 0;
    //[TODO] Read code from file
    println!(
        "{:?}",
        recursive_decent_parser(MatchStacks::default().START, &mut offset, "int fn_name(){}"),
    );
}
