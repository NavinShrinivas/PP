use regex::Regex;
use std::env;
use std::file;
use std::fmt;

// Define all "states" and their matching functions
// Possibly the worst possible recursive decent praser lol
// Consider this v0.0.0.0.0.0.0.0.0.1 of maybe something ill build seriously lol
// Obviously cannot handle left recursive or left factored
// [GENERIC] A grammer will always have states
#[derive(Debug)]
enum States {
    START,
    TYPE,
    ARGS,
    DECL,
}

// [GENERIC] for code logic
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

// Match states to their grammer defintion [GENERIC]
impl States {
    fn reverse_string_state_match_stacks(s: &str) -> Option<Vec<Vec<String>>> {
        match s {
            "START" => Some(MatchStacks::default().START),
            "TYPE" => Some(MatchStacks::default().TYPE),
            "ARGS" => Some(MatchStacks::default().ARGS),
            "DECL" => Some(MatchStacks::default().DECL),
            _ => None,
        }
    }
}

//[GENERIC] Defining the grammer
struct MatchStacks {
    START: Vec<Vec<String>>,
    TYPE: Vec<Vec<String>>,
    ARGS: Vec<Vec<String>>,
    DECL: Vec<Vec<String>>,
}

// [GENERIC] Defining the grammer and match rules :
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

fn regex_matches<'a>(
    regex_pat: &str,
    offset: usize,
    code: &'a str,
) -> (bool, &'a str, usize, usize) {
    let new_regex_matcher = Regex::new(regex_pat).unwrap();
    match new_regex_matcher.captures_at(code, offset) {
        Some(matches) => {
            return (
                true,
                matches.get(0).unwrap().as_str().clone(),
                matches.get(0).unwrap().end(),
                matches.get(0).unwrap().start(),
            )
        }
        None => return (false, "", 0, 0),
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
    let mut largest_offset = inital_offset as usize;
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
            match States::reverse_string_state_match_stacks(j.as_str()) {
                Some(stacks) => {
                    let response = recursive_decent_parser(stacks, &mut offset.clone(), code);
                    println!("{:?}", response);
                    if response.0 == true {
                        stack_reponse = response.0;
                        *offset = response.1;
                    } else if response.0 == false {
                        stack_reponse = false;
                        break;
                    }
                }
                None => {
                    //Greedy match using regex functions
                    let response = regex_matches(&j, offset.clone(), code);
                    println!("{} {:?}", j, response);
                    if response.0 == false {
                        stack_reponse = false;
                        break;
                    } else if response.3 == *offset {
                        stack_reponse = true;
                        *offset = response.2;
                        println!("{:?}", response);
                    } else {
                        stack_reponse = false;
                        break;
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
    // let mut file_name: String;
    // let args: Vec<String> = env::args().collect();
    // for (i, v) in args.iter().enumerate() {
    //     if v.to_string() == "file".to_string() {
    //         file_name = match args.get(i + 1) {
    //             Some(txt) => txt.to_string(),
    //             None => panic!("Godddamit, give me a file name!"),
    //         }
    //     }
    // }

    println!(
        "{:?}",
        recursive_decent_parser(
            MatchStacks::default().START,
            &mut offset,
            "int fn_name(int x,T y){}"
        ),
    );
}
