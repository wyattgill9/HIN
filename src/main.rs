use std::fs;

fn get_file_content(path: &str) -> std::io::Result<String> {
    let mut content = fs::read_to_string(path)?;
    content = content.replace("\n\n", "\\n");
    content = content.replace("\n", "");
    content = content.split_whitespace().collect();
    content = content.replace("\\n", "\n");
    content = content.replace("\n", " ");
    let content = " ".to_string() + &content;
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
            name: "".to_string(),
            parameters: vec!["".to_string()],
            body: "".to_string()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Program {
    functions: Vec<Function>,
}

fn parse_funct(content :&str) -> Function {
    let mut function = Function::default();
    function.name = content.to_string(); 
    function
}


fn parse_program(content: &str) -> Vec<Function> {
    let string_content = content.to_string();
    let mut functions = vec![];
    
    let mut i = 0;
    while let Some(pos) = string_content[i..].find(" @") {
        let start = i + pos;
        let mut end = start + 2;

        while end < string_content.len() && !string_content[end..end + 1].chars().next().unwrap().is_whitespace() {
            end += 1;
        }

        let function = parse_funct(&string_content[start..end]);
        functions.push(function);

        i = end;
    }
    functions
}

fn main() {
    let add_file_path = "./book/add.hvml";
    let file_content = get_file_content(&add_file_path).unwrap();
    println!("{}", file_content);
    let functions = parse_program(&file_content);
    println!("{:?}", functions);
}
