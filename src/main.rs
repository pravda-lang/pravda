use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    println!("Pravda 0.1.0");
    let library = &mut HashMap::from([
        (
            "+".to_string(),
            Function::Primitive(|params| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = *params.get(0).expect("The paramater is deficiency");
                for i in params[1..params.len()].to_vec().iter() {
                    result += i;
                }
                Type::Number(result)
            }),
        ),
        (
            "-".to_string(),
            Function::Primitive(|params| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = *params.get(0).expect("The paramater is deficiency");
                for i in params[1..params.len()].to_vec().iter() {
                    result -= i;
                }
                Type::Number(result)
            }),
        ),
        (
            "*".to_string(),
            Function::Primitive(|params| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = *params.get(0).expect("The paramater is deficiency");
                for i in params[1..params.len()].to_vec().iter() {
                    result *= i;
                }
                Type::Number(result)
            }),
        ),
        (
            "/".to_string(),
            Function::Primitive(|params| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = *params.get(0).expect("The paramater is deficiency");
                for i in params[1..params.len()].to_vec().iter() {
                    result /= i;
                }
                Type::Number(result)
            }),
        ),
        (
            "%".to_string(),
            Function::Primitive(|params| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();

                let mut result: f64 = *params.get(0).expect("The paramater is deficiency");
                for i in params[1..params.len()].to_vec().iter() {
                    result %= i;
                }
                Type::Number(result)
            }),
        ),
        (
            "print".to_string(),
            Function::Primitive(|params| {
                print!(
                    "{}",
                    params
                        .get(0)
                        .expect("The paramater is deficiency")
                        .get_string()
                );
                Type::Null
            }),
        ),
    ]);

    let memory = &mut HashMap::new();
    loop {
        let mut code = String::new();
        loop {
            let enter = input("> ").trim().to_string();
            if enter.is_empty() {
                break;
            }
            code += &format!("{enter}\n");
        }

        if !code.is_empty() {
            println!("{}", run(code, library, memory).get_string());
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut result = String::new();
    io::stdin().read_line(&mut result).ok();
    result.trim().to_string()
}

#[derive(Clone, Debug)]
enum Type {
    Code(Vec<Type>),
    Symbol(String),
    Number(f64),
    String(String),
    Bool(bool),
    Null,
}

impl Type {
    fn parse(source: String) -> Type {
        let mut source = source.trim().to_string();
        if let Ok(value) = source.parse::<f64>() {
            Type::Number(value)
        } else if let Ok(value) = source.parse::<bool>() {
            Type::Bool(value)
        } else if source == "null" {
            Type::Null
        } else if source.starts_with('"') && source.starts_with('"') {
            Type::String({
                source.remove(source.find('"').unwrap_or_default());
                source.remove(source.rfind('"').unwrap_or_default());
                source.to_string()
            })
        } else if source.starts_with("(") && source.ends_with(")") {
            Type::Code({
                source.remove(source.find("(").unwrap_or_default());
                source.remove(source.rfind(")").unwrap_or_default());
                tokenize(source)
                    .into_iter()
                    .map(|item| Type::parse(item.to_string()))
                    .collect()
            })
        } else {
            Type::Symbol(source.to_string())
        }
    }

    fn get_number(&self) -> f64 {
        match self {
            Type::Number(value) => *value,
            Type::String(value) | Type::Symbol(value) => value.parse().unwrap_or_default(),
            Type::Bool(value) => {
                if *value {
                    1.0
                } else {
                    0.0
                }
            }
            Type::Code(value) => value.get(0).unwrap_or(&Type::Null).get_number(),
            Type::Null => 0.0,
        }
    }

    fn get_string(&self) -> String {
        match self {
            Type::Number(value) => value.to_string(),
            Type::String(value) | Type::Symbol(value) => value.to_string(),
            Type::Bool(value) => value.to_string(),
            Type::Code(value) => format!(
                "({})",
                value
                    .iter()
                    .map(|i| i.get_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Type::Null => "null".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
enum Function {
    Primitive(fn(Vec<Type>) -> Type),
    UserDefined { args: Vec<Type>, program: String },
}

fn run(
    source: String,
    library: &mut HashMap<String, Function>,
    memory: &mut HashMap<String, Type>,
) -> Type {
    let source: Vec<&str> = source.split(";").collect();
    let mut result = Type::Null;
    for lines in source {
        if lines.contains(" = ") {
            let lines: Vec<&str> = lines.split(" = ").collect();
            let define = lines[0].split_whitespace().collect::<Vec<&str>>();
            if define.len() > 1 {
                library.insert(
                    define[0].to_string(),
                    Function::UserDefined {
                        args: define[1..define.len()]
                            .to_vec()
                            .iter()
                            .map(|i| Type::parse(i.to_string()))
                            .collect(),
                        program: lines[1..lines.len()].to_vec().join(" = "),
                    },
                );
            } else {
                result = eval(lines[1..lines.len()].to_vec().join(" = "), library, memory);
                memory.insert(define[0].to_string(), result.clone());
            }
        } else {
            result = eval(lines.to_string(), library, memory);
        }
    }
    result
}

fn eval(
    programs: String,
    library: &mut HashMap<String, Function>,
    memory: &mut HashMap<String, Type>,
) -> Type {
    let programs: Vec<Type> = tokenize(programs)
        .iter()
        .map(|i| Type::parse(i.to_owned()))
        .collect();
    if let Type::Symbol(identify) = programs[0].clone() {
        let params: Vec<Type> = programs[1..programs.len()]
            .to_vec()
            .iter()
            .map(|i| {
                if let Type::Code(code) = i.clone() {
                    eval(
                        {
                            let temp = Type::Code(code)
                                .get_string()
                                .trim()
                                .chars()
                                .collect::<Vec<char>>();
                            temp[1..temp.len() - 1]
                                .to_vec()
                                .iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<String>>()
                                .join("")
                        },
                        library,
                        memory,
                    )
                } else if let Type::Symbol(name) = i.clone() {
                    if let Some(value) = memory.get(&name) {
                        value.to_owned()
                    } else {
                        i.to_owned()
                    }
                } else {
                    i.to_owned()
                }
            })
            .collect();

        if let Some(liberal) = library.get(&identify) {
            if let Function::Primitive(function) = liberal {
                function(params)
            } else if let Function::UserDefined { args, program } = liberal {
                for (arg, value) in args.iter().zip(params) {
                    memory.insert(arg.get_string(), value);
                }
                eval(program.to_string(), library, memory)
            } else {
                todo!()
            }
        } else {
            if let Some(value) = memory.get(&identify) {
                value.to_owned()
            } else {
                programs[0].clone()
            }
        }
    } else {
        programs[0].to_owned()
    }
}

fn tokenize(input: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_parentheses: usize = 0;
    let mut in_quote = false;

    for c in input.chars() {
        match c {
            '(' if !in_quote => {
                if in_parentheses != 0 {
                    in_parentheses += 1;
                    current_token.push(c);
                } else {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                    in_parentheses += 1;
                    current_token.push(c);
                }
            }
            ')' if !in_quote => {
                if in_parentheses != 0 {
                    current_token.push(c);
                    in_parentheses -= 1;
                    if in_parentheses == 0 {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                } else {
                    panic!("Syntax error: invalid end of parentheses")
                }
            }
            '"' => {
                if in_parentheses == 0 {
                    if in_quote {
                        current_token.push(c);
                        in_quote = false;
                        tokens.push(current_token.clone());
                        current_token.clear();
                    } else {
                        in_quote = true;
                        current_token.push(c);
                    }
                } else {
                    current_token.push(c);
                }
            }
            ' ' | '\n' | '\t' | '\r' | '　' => {
                if in_parentheses != 0 || in_quote {
                    current_token.push(c);
                } else {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
            }
            _ => {
                current_token.push(c);
            }
        }
    }

    if in_parentheses != 0 {
        panic!("Syntax error: There isn't end of parentheses");
    }
    if in_quote {
        panic!("Syntax error: There isn't end of quote");
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}
