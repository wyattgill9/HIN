use nom::{
    bytes::complete::{tag, take_until, take_while1},
    combinator::map,
    sequence::{delimited, tuple, preceded},
    IResult,
};

use std::fs;

#[derive(Debug, PartialEq)]
pub struct Function {
    name: String,
    parameters: Vec<String>,
    body: String,
}

#[derive(Debug, PartialEq)]
pub struct Program {
    functions: Vec<Function>,
    main: Function,
}

fn get_file_content(path: &str) -> std::io::Result<String> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

fn identifier(input: &str) -> IResult<&str, String> {
    map(
        take_while1(|c: char| c.is_alphanumeric() || c == '_'),
        |s: &str| s.to_string(),
    )(input)
}

fn parameters(input: &str) -> IResult<&str, Vec<String>> {
    let (input, param_str) = delimited(tag("("), take_until(|c| c != ")"), tag(")"))(input)?;

    let param = param_str
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    Ok((input, param))
}

pub fn parse_function(input: &str) -> IResult<&str, Function> {
    map(
        tuple((
            preceded(char('@'), identifier),
            parameters,
            preceded(
                tuple((space0, char('='), space0)),
                take_while1(|_| true),
            ),
        )),
        |(name, parameters, body)| Function {
            name,
            parameters,
            body: body.trim().to_string(),
        },
    )(input)
}

pub fn parse_program(input: &str) -> Vec<Function> {
    let mut functions = Vec::new();
    let mut remaining = input;
    
    while !remaining.is_empty() {
        match parse_function(remaining) {
            Ok((rest, function)) => {
                functions.push(function);
                remaining = rest.trim_start();
            }
            Err(_) => {
                if let Some(pos) = remaining.find('@') {
                    if pos > 0 {
                        remaining = &remaining[pos..];
                    } else {
                        remaining = &remaining[1..];
                    }
                } else {
                    break;
                }
            }
        }
    }
    
    functions
}


fn main() {
    let add_file_path = "./book/add.hvml";
    let file_content = get_file_content(&add_file_path).unwrap();
    let functions = parse_program(&file_content);
    for function in functions {
        println!("Function: {}", function.name);
        println!("Parameters: {:?}", function.parameters);
        println!("Body: {}", function.body);
        println!("--------------------");
    }

}
