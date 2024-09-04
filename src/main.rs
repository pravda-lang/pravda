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
            "=".to_string(),
            Type::Function(Function::Primitive(|params| {
                Type::Bool({
                    let params: Vec<String> = params.iter().map(|i| i.get_symbol()).collect();
                    params.windows(2).all(|window| window[0] == window[1])
                })
            })),
        ),
        (
            "|".to_string(),
            Type::Function(Function::Primitive(|params| {
                Type::Bool({
                    let params: Vec<bool> = params.iter().map(|i| i.get_bool()).collect();
                    params.iter().any(|&x| x)
                })
            })),
        ),
        (
            "&".to_string(),
            Type::Function(Function::Primitive(|params| {
                Type::Bool({
                    let params: Vec<bool> = params.iter().map(|i| i.get_bool()).collect();
                    params.iter().all(|&x| x)
                })
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
                Type::String(input(&if let Some(prompt) = params.get(0) {
                    prompt.get_string()
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
        (
            "read-file".to_string(),
            Type::Function(Function::Primitive(|params| {
                if let Some(path) = params.get(0) {
                    let path = path.get_string(); // Create a binding to extend the lifetime
                    if let Ok(data) = read_to_string(Path::new(&path)) {
                        Type::String(data)
                    } else {
                        Type::Null
                    }
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "list".to_string(),
            Type::Function(Function::Primitive(|params| Type::List(params))),
        ),
        (
            "car".to_string(),
            Type::Function(Function::Primitive(|params| {
                if let Some(list) = params.get(0) {
                    if let Some(car) = list.get_list().get(0) {
                        car.clone()
                    } else {
                        Type::Null
                    }
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "cdr".to_string(),
            Type::Function(Function::Primitive(|params| {
                if let Some(list) = params.get(0) {
                    if list.get_list().len() >= 2 {
                        Type::List(list.get_list()[1..list.get_list().len()].to_vec())
                    } else {
                        Type::Null
                    }
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "len".to_string(),
            Type::Function(Function::Primitive(|params| {
                if let Some(Type::List(list)) = params.get(0) {
                    Type::Number(list.len() as f64)
                } else if let Some(Type::String(string)) = params.get(0) {
                    Type::Number(string.chars().count() as f64)
                } else {
                    Type::Null
                }
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
        println!("Pravda 0.5.3");
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
                println!("{}", run(code, memory).get_symbol());
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
            .map(|i| i.get_symbol())
            .collect::<Vec<String>>()
            .join("\n")
            == target
                .iter()
                .map(|i| i.get_symbol())
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
    List(Vec<Type>),
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
        } else if source.starts_with("(") && source.ends_with(")") && source.contains("->") {
            source.remove(source.find("(").unwrap_or_default());
            source.remove(source.rfind(")").unwrap_or_default());
            let define: Vec<&str> = source.split("->").collect();
            Type::Function(Function::UserDefined(vec![(
                define[0]
                    .split_whitespace()
                    .into_iter()
                    .map(|i| Type::parse(i.to_string()))
                    .collect(),
                (
                    define[1..define.len()].join("->").to_string(),
                    HashMap::new(),
                ),
            )]))
        } else if source.starts_with("(") && source.ends_with(")") {
            Type::Code({
                source.remove(source.find("(").unwrap_or_default());
                source.remove(source.rfind(")").unwrap_or_default());
                tokenize(source)
                    .into_iter()
                    .map(|item| Type::parse(item.to_string()))
                    .collect()
            })
        } else if source.starts_with("[") && source.ends_with("]") {
            Type::List({
                source.remove(source.find("[").unwrap_or_default());
                source.remove(source.rfind("]").unwrap_or_default());
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
            Type::Code(value) | Type::List(value) => {
                value.get(0).unwrap_or(&Type::Null).get_number()
            }
            Type::Null => 0.0,
            Type::Function(Function::UserDefined(value)) => value.len() as f64,
            Type::Function(Function::Primitive(_)) => 0.0,
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
            Type::List(value) => format!(
                "[{}]",
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

    fn get_symbol(&self) -> String {
        match self {
            Type::Number(value) => value.to_string(),
            Type::String(value) => format!("\"{}\"", value),
            Type::Symbol(value) => value.to_string(),
            Type::Bool(value) => value.to_string(),
            Type::Code(value) => format!(
                "({})",
                value
                    .iter()
                    .map(|i| i.get_symbol())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Type::List(value) => format!(
                "[{}]",
                value
                    .iter()
                    .map(|i| i.get_symbol())
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

    fn get_bool(&self) -> bool {
        match self {
            Type::Number(value) => *value != 0.0,
            Type::String(value) | Type::Symbol(value) => value.parse().unwrap_or_default(),
            Type::Bool(value) => *value,
            Type::Code(value) | Type::List(value) => value.get(0).unwrap_or(&Type::Null).get_bool(),
            Type::Null => false,
            Type::Function(_) => true,
        }
    }

    fn get_list(&self) -> Vec<Type> {
        match self {
            Type::List(value) => value.to_owned(),
            Type::String(value) => value
                .chars()
                .into_iter()
                .map(|c| Type::String(c.to_string()))
                .collect(),
            other => vec![other.to_owned()],
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
                    if exist[0].0.len() == args.len() {
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
                    } else {
                        eprintln!("Error! the function arguments length should be immutable");
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
        if programs.len() == 1 {
            programs[0].to_owned()
        } else {
            Type::List(programs)
        }
    }
}

fn call_function(function: Function, args: Vec<Type>, memory: &mut HashMap<String, Type>) -> Type {
    let mut params: Vec<Type> = vec![];
    for i in args {
        if let Type::Code(code) = i.clone() {
            params.push(eval(
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
            ))
        } else if let Type::Symbol(name) = i.clone() {
            if name.starts_with("*") {
                let name = name[1..name.len()].to_string();
                if let Some(value) = memory.get(&name) {
                    for j in value.get_list() {
                        params.push(j.to_owned())
                    }
                } else if let Type::List(list) = Type::parse(name.clone()) {
                    for j in list {
                        params.push(j.to_owned())
                    }
                } else if let Type::Code(code) = Type::parse(name.clone()) {
                    let result = eval(
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
                    );
                    for j in result.get_list() {
                        params.push(j.to_owned())
                    }
                } else {
                    params.push(Type::parse(name))
                }
            } else {
                if let Some(value) = memory.get(&name) {
                    params.push(value.to_owned())
                } else {
                    params.push(i.to_owned())
                }
            }
        } else {
            params.push(i.to_owned());
        }
    }

    if let Function::Primitive(function) = function {
        function(params)
    } else if let Function::UserDefined(object) = function {
        if let Some((program, scope)) = search_vec(object.clone(), params.clone()) {
            let mut scope = scope.clone();
            eval(program.to_string(), &mut scope)
        } else {
            if let Some((args, (program, scope))) = {
                let mut flag = None;
                for item in {
                    let mut object = object.clone();
                    object.reverse();
                    object
                } {
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
                let mut scope: &mut HashMap<String, Type> = &mut scope.clone();
                scope.extend(memory.to_owned());
                if args[args.len() - 1].get_symbol().starts_with("*") {
                    for (arg, value) in args.iter().zip(params.to_vec()) {
                        if arg.get_symbol().starts_with("*") {
                            scope.insert(
                                arg.get_symbol()[1..arg.get_symbol().len()].to_string(),
                                Type::List(
                                    params[params
                                        .iter()
                                        .position(|i| i.get_symbol() == value.get_symbol())
                                        .unwrap()
                                        ..params.len()]
                                        .to_vec(),
                                ),
                            );
                        } else {
                            scope.insert(arg.get_symbol(), value);
                        }
                    }
                } else {
                    for (arg, value) in args.iter().zip(params.to_vec()) {
                        scope.insert(arg.get_symbol(), value);
                    }
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
    let mut in_brackets: usize = 0;
    let mut in_quote = false;

    for c in input.chars() {
        match c {
            '(' if !in_quote => {
                if in_parentheses != 0 {
                    in_parentheses += 1;
                    current_token.push(c);
                } else {
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
            '[' if !in_quote => {
                if in_brackets != 0 {
                    in_brackets += 1;
                    current_token.push(c);
                } else {
                    in_brackets += 1;
                    current_token.push(c);
                }
            }
            ']' if !in_quote => {
                if in_brackets != 0 {
                    current_token.push(c);
                    in_brackets -= 1;
                    if in_brackets == 0 {
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
                if in_parentheses != 0 || in_brackets != 0 || in_quote {
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

    if !(in_parentheses != 0 || in_brackets != 0 || in_quote) && !current_token.is_empty() {
        tokens.push(current_token);
    }
    tokens
}
