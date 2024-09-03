use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    let memory: &mut HashMap<String, Type> = &mut HashMap::from([
        (
            "+".to_string(),
            Type::Function(Function::Primitive(|params| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = if let Some(result) = params.get(0) {
                    result.to_owned()
                } else {
                    return Type::Null;
                };
                for i in params[1..params.len()].to_vec().iter() {
                    result += i;
                }
                Type::Number(result)
            })),
        ),
        (
            "-".to_string(),
            Type::Function(Function::Primitive(|params| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                if params.len() == 1 {
                    Type::Number(-params[0])
                } else {
                    let mut result: f64 = if let Some(result) = params.get(0) {
                        result.to_owned()
                    } else {
                        return Type::Null;
                    };
                    for i in params[1..params.len()].to_vec().iter() {
                        result -= i;
                    }
                    Type::Number(result)
                }
            })),
        ),
        (
            "*".to_string(),
            Type::Function(Function::Primitive(|params| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = if let Some(result) = params.get(0) {
                    result.to_owned()
                } else {
                    return Type::Null;
                };
                for i in params[1..params.len()].to_vec().iter() {
                    result *= i;
                }
                Type::Number(result)
            })),
        ),
        (
            "/".to_string(),
            Type::Function(Function::Primitive(|params| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = if let Some(result) = params.get(0) {
                    result.to_owned()
                } else {
                    return Type::Null;
                };
                for i in params[1..params.len()].to_vec().iter() {
                    result /= i;
                }
                Type::Number(result)
            })),
        ),
        (
            "%".to_string(),
            Type::Function(Function::Primitive(|params| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = if let Some(result) = params.get(0) {
                    result.to_owned()
                } else {
                    return Type::Null;
                };
                for i in params[1..params.len()].to_vec().iter() {
                    result %= i;
                }
                Type::Number(result)
            })),
        ),
        (
            "^".to_string(),
            Type::Function(Function::Primitive(|params| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = if let Some(result) = params.get(0) {
                    result.to_owned()
                } else {
                    return Type::Null;
                };
                for i in params[1..params.len()].to_vec().iter() {
                    result = result.powf(*i);
                }
                Type::Number(result)
            })),
        ),
        (
            "concat".to_string(),
            Type::Function(Function::Primitive(|params| {
                let params: Vec<String> = params.iter().map(|i| i.get_string()).collect();
                Type::String(params.join(""))
            })),
        ),
        (
            "repeat".to_string(),
            Type::Function(Function::Primitive(|params| {
                Type::String(
                    if let Some(count) = params.get(0) {
                        count.get_string()
                    } else {
                        return Type::Null;
                    }
                    .repeat(if let Some(count) = params.get(1) {
                        count.get_number() as usize
                    } else {
                        return Type::Null;
                    }),
                )
            })),
        ),
        (
            "input".to_string(),
            Type::Function(Function::Primitive(|params| {
                Type::String(input(&if let Some(count) = params.get(0) {
                    count.get_string()
                } else {
                    "".to_string()
                }))
            })),
        ),
        (
            "print".to_string(),
            Type::Function(Function::Primitive(|params| {
                println!(
                    "{}",
                    if let Some(count) = params.get(0) {
                        count.get_string()
                    } else {
                        "".to_string()
                    }
                );
                Type::Null
            })),
        ),
    ]);

    let args: Vec<String> = args().collect();
    if args.len() >= 2 {
        if let Ok(code) = read_to_string(Path::new(&args[1])) {
            run(code, memory);
        } else {
            eprintln!("Error! it fault to open the script file")
        }
    } else {
        println!("Pravda 0.4.0");
        loop {
            let mut code = String::new();
            loop {
                let enter = input("> ").trim().to_string();
                if enter.is_empty() {
                    break;
                }
                code += &format!("{enter} ");
            }

            if !code.is_empty() {
                println!("{}", run(code, memory).get_string());
            }
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

fn search_vec(
    vec: Vec<(Vec<Type>, (String, HashMap<String, Type>))>,
    target: Vec<Type>,
) -> Option<(String, HashMap<String, Type>)> {
    let mut temp = None;
    for item in vec {
        if item
            .0
            .iter()
            .map(|i| i.get_string())
            .collect::<Vec<String>>()
            .join("\n")
            == target
                .iter()
                .map(|i| i.get_string())
                .collect::<Vec<String>>()
                .join("\n")
        {
            temp = Some(item.1);
        }
    }
    temp
}

#[derive(Clone, Debug)]
enum Type {
    Code(Vec<Type>),
    Symbol(String),
    Function(Function),
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
            _ => 0.0,
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
            Type::Function(Function::Primitive(function)) => {
                format!("<Built-in function: {:?}>", function)
            }
            Type::Function(Function::UserDefined(_)) => "<User-defined function>".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
enum Function {
    Primitive(fn(Vec<Type>) -> Type),
    UserDefined(Vec<(Vec<Type>, (String, HashMap<String, Type>))>),
}

fn run(source: String, memory: &mut HashMap<String, Type>) -> Type {
    let source: Vec<&str> = source.split(";").collect();
    let mut result = Type::Null;
    for lines in source {
        let lines = lines.trim().to_string();
        if lines.contains(" = ") {
            let lines: Vec<&str> = lines.split(" = ").collect();
            let define = lines[0].split_whitespace().collect::<Vec<&str>>();
            if define.len() > 1 {
                if let Some(Type::Function(Function::UserDefined(exist))) = memory.get(define[0]) {
                    let mut exist = exist.clone();
                    let args: Vec<Type> = define[1..define.len()]
                        .to_vec()
                        .iter()
                        .map(|i| Type::parse(i.to_string()))
                        .collect();
                    if exist.len() == args.len() {
                        exist.push((
                            args,
                            (
                                lines[1..lines.len()].to_vec().join(" = "),
                                memory.to_owned(),
                            ),
                        ));
                        let object = Type::Function(Function::UserDefined(exist));
                        result = object.clone();
                        memory.insert(define[0].to_string(), object);
                    }
                } else {
                    let object = Type::Function(Function::UserDefined(vec![(
                        define[1..define.len()]
                            .to_vec()
                            .iter()
                            .map(|i| Type::parse(i.to_string()))
                            .collect(),
                        (
                            lines[1..lines.len()].to_vec().join(" = "),
                            memory.to_owned(),
                        ),
                    )]));
                    result = object.clone();
                    memory.insert(define[0].to_string(), object);
                }
            } else {
                result = eval(lines[1..lines.len()].to_vec().join(" = "), memory);
                memory.insert(define[0].to_string(), result.clone());
            }
        } else {
            result = eval(lines.to_string(), memory);
        }
    }
    result
}

fn eval(programs: String, memory: &mut HashMap<String, Type>) -> Type {
    let programs: Vec<Type> = tokenize(programs)
        .iter()
        .map(|i| Type::parse(i.to_owned()))
        .collect();
    if programs.is_empty() {
        return Type::Null;
    }

    if let Type::Symbol(identify) = programs[0].clone() {
        if let Some(value) = memory.get(&identify) {
            if let Type::Function(name) = value {
                call_function(
                    name.to_owned(),
                    programs[1..programs.len()].to_vec(),
                    memory,
                )
            } else {
                value.to_owned()
            }
        } else {
            programs[0].clone()
        }
    } else if let Type::Function(liberal) = &programs[0] {
        call_function(
            liberal.clone(),
            programs[1..programs.len()].to_vec(),
            memory,
        )
    } else {
        programs[0].to_owned()
    }
}

fn call_function(
    function: Function,
    params: Vec<Type>,
    memory: &mut HashMap<String, Type>,
) -> Type {
    let params: Vec<Type> = params
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

    if let Function::Primitive(function) = function {
        function(params)
    } else if let Function::UserDefined(object) = function {
        if let Some((program, scope)) = search_vec(object.clone(), params.clone()) {
            let mut scope = scope.clone();
            eval(program.to_string(), &mut scope)
        } else {
            if let Some(object2) = {
                // 全て仮引数か?
                let mut flag = None;
                for item in object.clone() {
                    if item
                        .0
                        .iter()
                        .all(|i| if let Type::Symbol(_) = i { true } else { false })
                    {
                        flag = Some(item);
                        break;
                    }
                }
                flag
            } {
                let (args, program) = (object2.0, object2.1 .0);
                let mut scope: &mut HashMap<String, Type> = &mut object2.1 .1.clone();

                scope.extend(memory.to_owned());
                for (arg, value) in args.iter().zip(params.to_vec()) {
                    scope.insert(arg.get_string(), value);
                }

                if args.len() <= params.len() {
                    eval(program.to_string(), &mut scope)
                } else {
                    let mut object = object.clone();
                    object.push((
                        args[params.len()..args.len()].to_vec(),
                        (program.clone(), scope.to_owned()),
                    ));
                    Type::Function(Function::UserDefined(object))
                }
            } else {
                Type::Null
            }
        }
    } else {
        return Type::Null;
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

    if !(in_parentheses != 0 || in_quote) && !current_token.is_empty() {
        tokens.push(current_token);
    }
    tokens
}
