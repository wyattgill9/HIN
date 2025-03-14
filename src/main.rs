use std::fs;

fn get_file_content(path: &str) -> std::io::Result<String> {
    let mut content = fs::read_to_string(path)?;
    content = content.replace("\n", "\\n");
    Ok(content)
}

#[derive(Debug, PartialEq)]
pub struct Function {
    name: String,
    parameters: Vec<String>,
    body: String,
}

impl Default for Function {
    fn default() -> Self {
        Function {
            name: "Main".to_string(),
            parameters: vec!["".to_string()],
            body: "".to_string()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Program {
    functions: Vec<Function>,
    main: Function,
}

fn parse_funct(content :&str) -> Function {
    Function::default()
}


fn parse_program(content: &str) -> String {
    let parsed_content = content.to_string();
    for i in parsed_content.chars() {
        match i {
            '@' => parse_funct(parsed_content.get(i)),
            _ => {}
        }
    }
   
    content.to_string()  
}


fn main() {
    let add_file_path = "./book/add.hvml";
    let file_content = get_file_content(&add_file_path).unwrap();
    let parsed_program = parse_program(&file_content);
    println!("{}", parsed_program);
}
