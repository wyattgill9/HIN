use std::fs;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub output_type: Option<String>,
    pub body: String,
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub functions: Vec<Function>,
}

fn parse_program(input: &str) -> Program {
    let mut functions = Vec::new();
    for token in input.split('@').map(|s| s.trim()).filter(|s| !s.is_empty()) {
        if token.contains('{') {
            functions.push(parse_function_def(token));
        } else if token.contains('=') {
            functions.push(parse_function_assignment(token));
        } else {
            eprintln!("Unrecognized function definition: {}", token);
        }
    }
    Program { functions }
}

fn parse_function_def(token: &str) -> Function {
    let token = token.trim();

    let name_end = token.find('(').unwrap_or(token.len());
    let name = token[..name_end].trim().to_string();

    let param_start = token.find('(').unwrap();
    let param_end = token.find(')').unwrap();
    let params_str = &token[param_start + 1..param_end];
    let parameters: Vec<String> = params_str
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let mut output_type = None;
    let mut rest = token[param_end + 1..].trim();
    if rest.starts_with('~') {
        if let Some((otype, remaining)) = rest.split_once(' ') {
            output_type = Some(otype.to_string());
            rest = remaining.trim();
        } else {
            output_type = Some(rest.to_string());
            rest = "";
        }
    }

    let body_start = rest.find('{').unwrap();
    let body_end = rest.rfind('}').unwrap();
    let body = rest[body_start + 1..body_end].trim().to_string();

    Function {
        name,
        parameters,
        output_type,
        body,
    }
}

fn parse_function_assignment(token: &str) -> Function {
    let token = token.trim();
    let parts: Vec<&str> = token.split('=').collect();
    let name = parts[0].trim().to_string();
    let body = parts[1].trim().to_string();
    Function {
        name,
        parameters: Vec::new(),
        output_type: None,
        body,
    }
}

fn get_file_content(path: &str) -> std::io::Result<String> {
    fs::read_to_string(path)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = get_file_content("./book/add.hvml").unwrap();
    let program = parse_program(&file_content);
    println!("{:#?}", program);
    Ok(())
}

