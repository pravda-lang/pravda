//! This is interpreter of Pravda programming language
use clap::Parser;
use dirs::home_dir;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict};
use rustyline::DefaultEditor;
use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;
use std::path::Path;

const VERSION: &str = "0.7.2";

#[derive(Parser, Debug)]
#[command(
    name = "Pravda",
    version = VERSION,
    author = "梶塚太智 <kajizukataichi@outlook.jp>",
    about = "A functional programming language that best of both worlds between Haskell and Lisp",
    after_help = "For more information, visit https://pravda-lang.github.io/",
)]
struct Cli {
    /// Run the script file
    #[arg(index = 1)]
    file: Option<String>,

    /// Command-line arguments to pass the script
    #[arg(index = 2, value_name="ARGS", num_args = 0..)]
    args_position: Option<Vec<String>>,

    /// Optional command-line arguments
    #[arg(short='a', long="args", value_name="ARGS", num_args = 0..)]
    args_option: Option<Vec<String>>,

    /// Run passed string as code
    #[arg(short = 'l', value_name = "ONE LINER", long)]
    one_liner: Option<String>,
}

/// The entry point
fn main() {
    let memory = &mut builtin_functions();

    let cli = Cli::parse();
    if let (Some(args), _) | (_, Some(args)) = (cli.args_option, cli.args_position) {
        memory.insert(
            "args".to_string(),
            Type::List(args.iter().map(|i| Type::String(i.to_owned())).collect()),
        );
    }

    if let Some(path) = cli.file {
        // Run from script file
        if let Ok(code) = read_to_string(Path::new(&path)) {
            run_program(code, memory);
        } else {
            eprintln!("Error! it fault to open the script file")
        }
    } else if let Some(code) = cli.one_liner {
        // Run from one-liner code
        println!("{}", run_program(code, memory).get_symbol());
    } else {
        println!("Pravda {VERSION}");
        let mut rl = DefaultEditor::new().unwrap();

        // REPL
        loop {
            let mut code = String::new();
            loop {
                let enter = rl.readline("> ").unwrap_or_default().trim().to_string();
                if enter.is_empty() {
                    break;
                }
                code += &format!("{enter} ");
            }

            if !code.is_empty() {
                println!("{}", run_program(code, memory).get_symbol());
            }
        }
    }
}

fn builtin_functions() -> HashMap<String, Type> {
    HashMap::from([
        ("new-line".to_string(), Type::String("\n".to_string())),
        ("tab".to_string(), Type::String("\t".to_string())),
        ("double-quote".to_string(), Type::String("\"".to_string())),
        (
            "+".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = if let Some(result) = params.first() {
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
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                if params.len() == 1 {
                    Type::Number(-params[0])
                } else {
                    let mut result: f64 = if let Some(result) = params.first() {
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
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = if let Some(result) = params.first() {
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
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = if let Some(result) = params.first() {
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
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = if let Some(result) = params.first() {
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
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = if let Some(result) = params.first() {
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
            "equal".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Type::Bool({
                    let params: Vec<String> = params.iter().map(|i| i.get_symbol()).collect();
                    params.windows(2).all(|window| window[0] == window[1])
                })
            })),
        ),
        (
            "less-than".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Type::Bool({
                    let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                    params.windows(2).all(|window| window[0] < window[1])
                })
            })),
        ),
        (
            "greater-than".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Type::Bool({
                    let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                    params.windows(2).all(|window| window[0] > window[1])
                })
            })),
        ),
        (
            "or".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Type::Bool({
                    let params: Vec<bool> = params.iter().map(|i| i.get_bool()).collect();
                    params.iter().any(|&x| x)
                })
            })),
        ),
        (
            "and".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Type::Bool({
                    let params: Vec<bool> = params.iter().map(|i| i.get_bool()).collect();
                    params.iter().all(|&x| x)
                })
            })),
        ),
        (
            "not".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if !params.is_empty() {
                    Type::Bool(!params[0].get_bool())
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "concat".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<String> = params.iter().map(|i| i.get_string()).collect();
                Type::String(params.join(""))
            })),
        ),
        (
            "split".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Type::List({
                    let text = if let Some(count) = params.first() {
                        count.get_string().to_owned()
                    } else {
                        return Type::Null;
                    };
                    let key = if let Some(count) = params.get(1) {
                        count.get_string()
                    } else {
                        return Type::Null;
                    };

                    text.split(&key)
                        .map(|i| Type::String(i.to_string()))
                        .collect()
                })
            })),
        ),
        (
            "input".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Type::String({
                    let mut rl = DefaultEditor::new().unwrap();
                    rl.readline(&if let Some(prompt) = params.first() {
                        prompt.get_string()
                    } else {
                        "".to_string()
                    })
                    .unwrap_or_default()
                })
            })),
        ),
        (
            "print".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                print!(
                    "{}",
                    params
                        .iter()
                        .map(|i| i.get_string())
                        .collect::<Vec<String>>()
                        .join("")
                );
                Type::Null
            })),
        ),
        (
            "list".to_string(),
            Type::Function(Function::BuiltIn(|params, _| Type::List(params))),
        ),
        (
            "car".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if let Some(list) = params.first() {
                    if let Some(car) = list.get_list().first() {
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
            Type::Function(Function::BuiltIn(|params, _| {
                if let Some(list) = params.first() {
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
            Type::Function(Function::BuiltIn(|params, _| {
                if let Some(Type::List(list)) = params.first() {
                    Type::Number(list.len() as f64)
                } else if let Some(Type::String(string)) = params.first() {
                    Type::Number(string.chars().count() as f64)
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "range".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() == 1 {
                    let mut range: Vec<Type> = vec![];
                    let mut current: f64 = 0.0;
                    while current < params[0].get_number() {
                        range.push(Type::Number(current));
                        current += 1.0;
                    }
                    Type::List(range)
                } else if params.len() == 2 {
                    let mut range: Vec<Type> = vec![];
                    let mut current: f64 = params[0].get_number();
                    while current < params[1].get_number() {
                        range.push(Type::Number(current));
                        current += 1.0;
                    }
                    Type::List(range)
                } else if params.len() >= 3 {
                    let mut range: Vec<Type> = vec![];
                    let mut current: f64 = params[0].get_number();
                    while current < params[1].get_number() {
                        range.push(Type::Number(current));
                        current += params[2].get_number();
                    }
                    Type::List(range)
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "map".to_string(),
            Type::Function(Function::BuiltIn(|params, memory| {
                if params.len() >= 2 {
                    let func = if let Type::Function(func) = params[1].clone() {
                        func
                    } else {
                        return Type::Null;
                    };
                    let memory = memory.clone();
                    Type::List(
                        params[0]
                            .get_list()
                            .iter()
                            .map(|i| call_function(func.clone(), vec![i.clone()], &memory))
                            .collect(),
                    )
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "filter".to_string(),
            Type::Function(Function::BuiltIn(|params, memory| {
                if params.len() >= 2 {
                    let func = if let Type::Function(func) = params[1].clone() {
                        func
                    } else {
                        return Type::Null;
                    };
                    let memory = memory.clone();
                    let mut result = Vec::new();

                    for item in params[0].get_list() {
                        if call_function(func.clone(), vec![item.clone()], &memory).get_bool() {
                            result.push(item);
                        }
                    }
                    Type::List(result)
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "reduce".to_string(),
            Type::Function(Function::BuiltIn(|params, memory| {
                if params.len() >= 3 {
                    let func = if let Type::Function(func) = params[2].clone() {
                        func
                    } else {
                        return Type::Null;
                    };
                    let variable = if let Type::Symbol(variable) = params[1].clone() {
                        variable
                    } else {
                        return Type::Null;
                    };

                    let mut memory = memory.clone();
                    let mut result = memory.get(&variable).unwrap_or(&Type::Null).to_owned();

                    for item in params[0].get_list() {
                        result = call_function(func.clone(), vec![item.clone()], &memory);
                        memory.insert(variable.clone(), result.clone());
                    }
                    result
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "for".to_string(),
            Type::Function(Function::BuiltIn(|params, memory| {
                if params.len() >= 2 {
                    let func = if let Type::Function(func) = params[1].clone() {
                        func
                    } else {
                        return Type::Null;
                    };
                    let memory = memory.clone();

                    let mut temp = Type::Null;
                    for item in params[0].get_list() {
                        temp = call_function(func.clone(), vec![item.clone()], &memory);
                    }
                    temp
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "while".to_string(),
            Type::Function(Function::BuiltIn(|params, memory| {
                if params.len() >= 2 {
                    let cond = if let Type::Expr(expr) = params[0].clone() {
                        expr
                    } else {
                        return Type::Null;
                    };
                    let block = if let Type::Block(block) = params[1].clone() {
                        block
                    } else {
                        return Type::Null;
                    };

                    let mut memory = memory.clone();
                    let mut temp = Type::Null;
                    while eval_expr(cond.clone(), &memory).get_bool() {
                        temp = run_program(block.clone(), &mut memory);
                    }
                    temp
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "if".to_string(),
            Type::Function(Function::BuiltIn(|params, memory| {
                let mut memory = memory.clone();
                if params.len() >= 2 {
                    if params[0].get_bool() {
                        match params[1].clone() {
                            Type::Expr(code) => eval_expr(code, &memory),
                            Type::Block(block) => run_program(block, &mut memory),
                            other => other,
                        }
                    } else if params.len() >= 3 {
                        match params[2].clone() {
                            Type::Expr(code) => eval_expr(code, &memory),
                            Type::Block(block) => run_program(block, &mut memory),
                            other => other,
                        }
                    } else {
                        Type::Null
                    }
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "eval".to_string(),
            Type::Function(Function::BuiltIn(|params, memory| {
                let mut memory = memory.clone();
                if !params.is_empty() {
                    match params[0].clone() {
                        Type::Expr(code) => eval_expr(code, &memory),
                        Type::Block(block) => run_program(block, &mut memory),
                        Type::Symbol(name) => memory.get(&name).unwrap_or(&Type::Null).to_owned(),
                        other => other,
                    }
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "load".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if !params.is_empty() {
                    let identify = params[0].get_string();

                    if let (_, Ok(code)) | (Ok(code), _) = (
                        read_to_string(Path::new(&identify.clone())),
                        read_to_string(home_dir().unwrap().join(Path::new(&identify.clone()))),
                    ) {
                        if params[0].get_string().ends_with(".pvd") {
                            Type::Function(Function::Module(code))
                        } else if params[0].get_string().ends_with(".py") {
                            let lines: Vec<String> =
                                code.split("\n").map(|s| s.to_string()).collect();
                            let (depent, code): (Vec<String>, String) = (
                                {
                                    if lines[0].to_string().trim().starts_with("import ") {
                                        let import: Vec<String> = lines[0]
                                            .replace("import ", "")
                                            .split(",")
                                            .map(|i| i.trim().to_string())
                                            .collect();
                                        import
                                    } else {
                                        vec![]
                                    }
                                },
                                code,
                            );
                            Type::Function(Function::Python(code, depent))
                        } else {
                            Type::Null
                        }
                    } else {
                        Type::Null
                    }
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "cmd-args".to_string(),
            Type::List(args().map(Type::String).collect()),
        ),
        (
            "cast".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() >= 2 {
                    match params[1].get_string().as_str() {
                        "string" => Type::String(params[0].get_string()),
                        "number" => Type::Number(params[0].get_number()),
                        "symbol" => Type::Symbol(params[0].get_symbol()),
                        "list" => Type::List(params[0].get_list()),
                        "bool" => Type::Bool(params[0].get_bool()),
                        _ => Type::Null,
                    }
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "type".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if !params.is_empty() {
                    match params[0] {
                        Type::Number(_) => Type::String("number".to_string()),
                        Type::String(_) => Type::String("string".to_string()),
                        Type::Bool(_) => Type::String("bool".to_string()),
                        Type::List(_) => Type::String("list".to_string()),
                        Type::Expr(_) => Type::String("expr".to_string()),
                        Type::Block(_) => Type::String("block".to_string()),
                        Type::Symbol(_) => Type::String("symbol".to_string()),
                        Type::Function(_) => Type::String("function".to_string()),
                        Type::Null => Type::String("null".to_string()),
                    }
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "exit".to_string(),
            Type::Function(Function::BuiltIn(|_, _| {
                std::process::exit(0);
            })),
        ),
    ])
}

/// Dynamic data type used in Pravda
#[derive(Clone, Debug)]
enum Type {
    /// Expression
    ///
    /// Example:
    /// ```
    /// (+ 1 2 (* 3 4))
    /// ```
    Expr(String),
    /// Code block
    ///
    /// Example:
    /// ```
    /// {
    ///     x = 5;
    ///     + x 1
    /// }
    /// ```
    Block(String),
    /// Symbol
    /// Used in such a variable, lazy evaluting, etc
    ///
    /// Example:
    /// ```
    /// lazy(+ 1 2)
    /// ```
    Symbol(String),
    /// Function object
    ///
    /// Example:
    /// ```
    /// lambda(n -> * n 2)
    /// ```
    Function(Function),
    /// Number that's 64bit float
    ///
    /// Example:
    /// ```
    /// 3.14
    /// ```
    Number(f64),
    /// Number that's 64bit float
    ///
    /// Example:
    /// ```
    /// "hello"
    /// ```
    String(String),
    /// Bool value
    /// Used in logical operating
    ///
    /// Example:
    /// ```
    /// true
    /// ```
    Bool(bool),
    /// List includes several values
    ///
    /// Example:
    /// ```
    /// [1 2 "abc"]
    /// ```
    List(Vec<Type>),
    /// Null
    /// Shows there's nothing
    ///
    /// Example:
    /// ```
    /// null
    /// ```
    Null,
}

impl Type {
    /// Parse from string
    fn parse(source: String) -> Type {
        let mut source = source.trim().to_string();
        if let Ok(value) = source.parse::<f64>() {
            // Number value
            Type::Number(value)
        } else if let Ok(value) = source.parse::<bool>() {
            // Bool value
            Type::Bool(value)
        } else if source == "null" {
            // Null value
            Type::Null
        } else if source.starts_with('"') && source.starts_with('"') {
            // String object is surrounded by double quote
            Type::String({
                source.remove(source.find('"').unwrap_or_default());
                source.remove(source.rfind('"').unwrap_or_default());
                source.to_string()
            })
        } else if (source.starts_with("lambda(") || source.starts_with(r"\("))
            && source.ends_with(")")
            && source.contains("->")
        {
            // Lambda expression
            source = source.replacen("lambda(", "", 1);
            source = source.replacen(r"\(", "", 1);
            source.remove(source.rfind(")").unwrap_or_default());
            let define: Vec<&str> = source.split("->").collect();
            Type::Function(Function::UserDefined(vec![(
                tokenize_expr(define[0].to_string())
                    .iter()
                    .map(|i| Type::parse(i.to_string()))
                    .collect(),
                (
                    define[1..define.len()].join("->").to_string(),
                    HashMap::new(),
                ),
            )]))
        } else if source.starts_with("(") && source.ends_with(")") {
            // Inner expression is surrounded by parentheses
            Type::Expr({
                source.remove(source.find("(").unwrap_or_default());
                source.remove(source.rfind(")").unwrap_or_default());
                source
            })
        } else if source.starts_with("{") && source.ends_with("}") {
            // Code block is surrounded by brace
            Type::Block({
                source.remove(source.find("{").unwrap_or_default());
                source.remove(source.rfind("}").unwrap_or_default());
                source
            })
        } else if source.starts_with("[") && source.ends_with("]") {
            // List object is surrounded by bracket
            Type::List({
                source.remove(source.find("[").unwrap_or_default());
                source.remove(source.rfind("]").unwrap_or_default());
                tokenize_expr(source)
                    .iter()
                    .map(|item| Type::parse(item.to_string()))
                    .collect()
            })
        } else {
            // Other value will be symbol
            Type::Symbol(source.to_string())
        }
    }

    fn from_python(result: &PyAny) -> Type {
        if let Ok(value) = result.extract::<f64>() {
            Type::Number(value)
        } else if let Ok(value) = result.extract::<String>() {
            Type::String(value)
        } else if let Ok(value) = result.extract::<bool>() {
            Type::Bool(value)
        } else if let Ok(value) = result.extract::<Vec<&PyAny>>() {
            Type::List(value.iter().map(|i| Type::from_python(i)).collect())
        } else {
            Type::Null
        }
    }

    fn get_number(&self) -> f64 {
        match self {
            Type::Number(value) => *value,
            Type::String(value) | Type::Symbol(value) => value.trim().parse().unwrap_or_default(),
            Type::Bool(value) => {
                if *value {
                    1.0
                } else {
                    0.0
                }
            }
            Type::List(value) => value.first().unwrap_or(&Type::Null).get_number(),
            Type::Null => 0.0,
            Type::Function(Function::UserDefined(value)) => value.len() as f64,
            Type::Function(Function::Python(value, _))
            | Type::Function(Function::Module(value)) => value.len() as f64,
            Type::Function(Function::BuiltIn(_)) => 0.0,
            Type::Expr(value) | Type::Block(value) => value.len() as f64,
        }
    }

    fn get_string(&self) -> String {
        match self {
            Type::Number(value) => value.to_string(),
            Type::String(value) | Type::Symbol(value) => value.to_string(),
            Type::Bool(value) => value.to_string(),
            Type::Expr(value) => format!("({})", value),
            Type::List(value) => format!(
                "[{}]",
                value
                    .iter()
                    .map(|i| i.get_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Type::Null => String::new(),
            Type::Function(Function::BuiltIn(function)) => {
                format!("<Built-in function: {:?}>", function)
            }
            Type::Function(Function::UserDefined(value)) => {
                format!(
                    "<User-defined function: ({})>",
                    value
                        .iter()
                        .last()
                        .unwrap()
                        .0
                        .iter()
                        .map(|i| i.get_symbol())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            Type::Block(value) => format!("{{ {} }}", value),
            Type::Function(Function::Python(path, _)) => format!("<Python function: {path}>"),
            Type::Function(Function::Module(path)) => format!("<Module function: {path}>"),
        }
    }

    fn get_symbol(&self) -> String {
        match self {
            Type::Number(value) => value.to_string(),
            Type::String(value) => format!("\"{}\"", value),
            Type::Symbol(value) => value.to_string(),
            Type::Bool(value) => value.to_string(),
            Type::Expr(value) => format!("({})", value),
            Type::List(value) => format!(
                "[{}]",
                value
                    .iter()
                    .map(|i| i.get_symbol())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Type::Null => "null".to_string(),
            Type::Function(Function::BuiltIn(function)) => {
                format!("<Built-in function: {:?}>", function)
            }
            Type::Function(Function::UserDefined(value)) => {
                format!(
                    "<User-defined function: ({})>",
                    value
                        .iter()
                        .last()
                        .unwrap()
                        .0
                        .iter()
                        .map(|i| i.get_symbol())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            Type::Block(value) => format!("{{ {} }}", value),
            Type::Function(Function::Python(path, _)) => format!("<Python function: {path}>"),
            Type::Function(Function::Module(path)) => format!("<Module function: {path}>"),
        }
    }

    fn get_bool(&self) -> bool {
        match self {
            Type::Number(value) => *value != 0.0,
            Type::String(value) | Type::Symbol(value) => value.trim().parse().unwrap_or_default(),
            Type::Bool(value) => *value,
            Type::List(value) => value.first().unwrap_or(&Type::Null).get_bool(),
            Type::Null => false,
            Type::Function(_) => true,
            Type::Expr(value) | Type::Block(value) => !value.is_empty(),
        }
    }

    fn get_list(&self) -> Vec<Type> {
        match self {
            Type::List(value) => value.to_owned(),
            Type::String(value) => value.chars().map(|c| Type::String(c.to_string())).collect(),
            other => vec![other.to_owned()],
        }
    }

    fn to_pyobj(&self) -> String {
        match self {
            Type::Number(value) => value.to_string(),
            Type::String(value) => format!(
                "\"{}\"",
                value
                    .replace("\n", "\\n")
                    .replace("\t", "\\t")
                    .replace("\"", &format!("\\{}", "\""))
            ),
            Type::Symbol(value) => value.to_string(),
            Type::Bool(value) => {
                let temp = value.to_string().chars().collect::<Vec<char>>();
                format!(
                    "{}{}",
                    temp[0].is_uppercase(),
                    temp[1..temp.len()].iter().collect::<String>()
                )
            }
            Type::List(value) => format!(
                "[{}]",
                value
                    .iter()
                    .map(|i| i.to_pyobj())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Type::Null => "None".to_string(),
            _ => "()".to_string(),
        }
    }
}

/// Function object used in the Pravda
#[derive(Clone, Debug)]
enum Function {
    /// Built-in function written in Rust code
    BuiltIn(
        fn(
            Vec<Type>,             // Passed arguments when it is calling
            HashMap<String, Type>, // Memory of variables and functions to access in the calling
        ) -> Type,
    ),
    /// User-defined function written in Pravda code
    UserDefined(
        Vec<(
            Vec<Type>, // The argument pattern and become the key
            (
                String,                // A program code of the function
                HashMap<String, Type>, // Memory of variables and functions to access in the calling
            ),
        )>,
    ),
    /// Python library function
    Python(String, Vec<String>),
    /// Pravda module function
    Module(String),
}

/// Run the program and return result value
/// # Arguments
/// * `source` - The source code string to run as program
/// * `memory` - Has functions and variables to access in the program
/// # Return values
/// This functions returns value that's result of running
fn run_program(source: String, memory: &mut HashMap<String, Type>) -> Type {
    let source = tokenize_program(source);
    let mut result = Type::Null;

    // Execute each line
    for lines in source {
        if lines.len() == 2 {
            let define = tokenize_expr(lines[0].to_string());
            // Is the line includes `=` ?
            if define.len() > 1 {
                if let Some(Type::Function(Function::UserDefined(exist))) = memory.get(&define[0]) {
                    let mut exist = exist.clone();
                    // Prepare argument
                    let args: Vec<Type> = define[1..define.len()]
                        .to_vec()
                        .iter()
                        .map(|i| eval_expr(i.to_string(), memory))
                        .collect();
                    if exist[0].0.len() == args.len() {
                        // Add pattern match of the function
                        exist.push((args, (lines[1].clone(), memory.to_owned())));
                        let object = Type::Function(Function::UserDefined(exist));
                        result = object.clone();
                        memory.insert(define[0].to_string(), object);
                    } else {
                        eprintln!("Error! the function arguments length should be immutable");
                    }
                } else {
                    // Define new function
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
                // Define variable
                result = eval_expr(lines[1..lines.len()].to_vec().join(" = "), memory);
                memory.insert(define[0].to_string(), result.clone());
            }
        } else {
            // Evaluate the expression
            result = eval_expr(lines[0].to_string(), memory);
        }
    }
    result
}

/// return 2 length vector splitted by it if the line has `=` else just the line in the top vector
fn tokenize_program(input: String) -> Vec<Vec<String>> {
    let mut tokens: Vec<Vec<String>> = Vec::new();
    let mut current_token = String::new();
    let mut after_equal = String::new();
    let mut is_equal = false;
    let mut in_parentheses: usize = 0;
    let mut in_quote = false;

    for c in input.chars() {
        match c {
            '{' if !in_quote => {
                if is_equal {
                    after_equal.push(c);
                } else {
                    current_token.push(c);
                }
                in_parentheses += 1;
            }
            '}' if !in_quote => {
                if is_equal {
                    after_equal.push(c);
                } else {
                    current_token.push(c);
                }
                in_parentheses -= 1;
            }
            ';' if !in_quote => {
                if in_parentheses != 0 {
                    if is_equal {
                        after_equal.push(c);
                    } else {
                        current_token.push(c);
                    }
                } else if !current_token.is_empty() {
                    if is_equal {
                        is_equal = false;
                        tokens.push(vec![current_token.clone(), after_equal.clone()]);
                        current_token.clear();
                        after_equal.clear();
                    } else {
                        tokens.push(vec![current_token.clone()]);
                        current_token.clear();
                    }
                }
            }
            '=' if !in_quote => {
                if in_parentheses != 0 {
                    if is_equal {
                        after_equal.push(c);
                    } else {
                        current_token.push(c);
                    }
                } else {
                    is_equal = true;
                }
            }
            '"' => {
                in_quote = !in_quote;
                if is_equal {
                    after_equal.push(c);
                } else {
                    current_token.push(c);
                }
            }
            _ => {
                if is_equal {
                    after_equal.push(c);
                } else {
                    current_token.push(c);
                }
            }
        }
    }

    if in_parentheses == 0 && !current_token.is_empty() {
        if is_equal {
            tokens.push(vec![current_token.clone(), after_equal]);
            current_token.clear();
        } else {
            tokens.push(vec![current_token.clone()]);
            current_token.clear();
        }
    }
    tokens
}

/// Evaluate the expression and return result value
/// # Arguments
/// * `expr` - The expression string to evaluate
/// * `memory` - Has functions and variables to access in the expression
/// # Return values
/// This functions returns value that's result of evaluating
fn eval_expr(expr: String, memory: &HashMap<String, Type>) -> Type {
    // Parse expression
    let expr: Vec<Type> = tokenize_expr(expr)
        .iter()
        .map(|i| Type::parse(i.to_owned()))
        .collect();

    if expr.is_empty() {
        return Type::Null;
    }

    if let Type::Symbol(identify) = expr[0].clone() {
        if let Some(value) = memory.get(&identify) {
            // Read memory value
            if let Type::Function(name) = value {
                call_function(name.to_owned(), expr[1..expr.len()].to_vec(), memory)
            } else {
                value.to_owned()
            }
        } else if Path::new(&identify.clone()).exists()
            || home_dir()
                .unwrap()
                .join(Path::new(&identify.clone()))
                .exists()
        {
            let result = run_program(format!("load {identify}"), &mut memory.clone());
            if let Type::Function(func) = result {
                call_function(func, expr[1..expr.len()].to_vec(), memory)
            } else {
                result
            }
        } else {
            expr[0].clone()
        }
    } else if let Type::Function(liberal) = &expr[0] {
        call_function(liberal.clone(), expr[1..expr.len()].to_vec(), memory)
    } else if let Type::Block(block) = &expr[0] {
        // Evaluate the code block
        let result = run_program(block.to_owned(), &mut memory.clone());
        if let Type::Function(func) = result {
            // If it's function, call it
            call_function(func, expr[1..expr.len()].to_vec(), &memory.clone())
        } else {
            result
        }
    } else if let Type::Expr(code) = &expr[0] {
        // Evaluate the expression
        let result = eval_expr(code.to_owned(), &memory.clone());
        if let Type::Function(func) = result {
            // If it's function, call it
            call_function(func, expr[1..expr.len()].to_vec(), &memory.clone())
        } else {
            result
        }
    } else if expr.len() == 1 {
        expr[0].to_owned()
    } else {
        // If there's multiple value, return it as a list
        Type::List(expr)
    }
}

/// Tokenize for the expression
/// ```
/// let result = tokenize_expr("+ 1 2 (* 3 4)")
/// assert_eq!(result, vec!["+", "1", "2", "(* 3 4)"]);
/// ```
fn tokenize_expr(input: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_parentheses: usize = 0;
    let mut in_quote = false;

    for c in input.chars() {
        match c {
            '(' if !in_quote => {
                in_parentheses += 1;
                current_token.push(c);
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
                in_parentheses += 1;
                current_token.push(c);
            }
            ']' if !in_quote => {
                if in_parentheses != 0 {
                    current_token.push(c);
                    in_parentheses -= 1;
                    if in_parentheses == 0 {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
            }
            '{' if !in_quote => {
                in_parentheses += 1;
                current_token.push(c);
            }
            '}' if !in_quote => {
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
                } else if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            _ => {
                current_token.push(c);
            }
        }
    }

    if !(in_parentheses != 0 || in_quote || current_token.is_empty()) {
        tokens.push(current_token);
    }
    tokens
}

/// Call ordered function and return result value
/// # Arguments
/// * `function` - The function object to call
/// * `args` - Several arguments that will be passed to function
/// * `memory` - Has functions and variables to access in the calling
/// # Return values
/// This functions returns value that's result of calling
fn call_function(function: Function, args: Vec<Type>, memory: &HashMap<String, Type>) -> Type {
    let mut params: Vec<Type> = vec![];
    for i in args {
        // Prepare arguments
        if let Type::Expr(code) = i.clone() {
            params.push(eval_expr(code, &memory.clone()))
        } else if let Type::Block(block) = i.clone() {
            params.push(run_program(block, &mut memory.clone()))
        } else if let Type::Symbol(name) = i.clone() {
            if name.starts_with("~") {
                // Processing of mutable length argument
                let name = name[1..name.len()].to_string();
                let value = Type::parse(name.clone());
                if let Some(value) = memory.get(&name) {
                    for j in value.get_list() {
                        // Expand　the list as argument
                        params.push(j.to_owned())
                    }
                } else if let Type::List(list) = value {
                    for j in list {
                        // Expand　the list as argument
                        params.push(j.to_owned())
                    }
                } else if let Type::Expr(code) = value {
                    let result = eval_expr(code, &memory.clone());
                    for j in result.get_list() {
                        // Expand　the list as argument
                        params.push(j.to_owned())
                    }
                } else if let Type::Block(code) = value {
                    // Run the code
                    let result = run_program(code, &mut memory.clone());
                    for j in result.get_list() {
                        // Expand　the list as argument
                        params.push(j.to_owned())
                    }
                } else {
                    params.push(value)
                }
            } else if name.starts_with("@") {
                // Processing of lazy evaluate expression
                params.push(Type::parse(name[1..name.len()].to_string()))
            } else if name.starts_with("lazy") {
                // Processing of lazy evaluate expression
                params.push(Type::parse(name["lazy".len()..name.len()].to_string()))
            } else if let Some(value) = memory.get(&name) {
                if value.get_symbol().starts_with("@") {
                    // Processing of lazy evaluate variable
                    let value =
                        Type::parse(value.get_symbol()[1..value.get_symbol().len()].to_string());
                    params.push(value)
                } else if value.get_symbol().starts_with("lazy") {
                    // Processing of lazy evaluate variable
                    let value = Type::parse(
                        value.get_symbol()["lazy".len()..value.get_symbol().len()].to_string(),
                    );
                    params.push(value)
                } else {
                    params.push(value.to_owned())
                }
            } else {
                params.push(i.to_owned())
            }
        } else {
            params.push(i.to_owned());
        }
    }

    if let Function::BuiltIn(function) = function {
        function(params, memory.to_owned())
    } else if let Function::UserDefined(object) = function {
        if let Some((program, scope)) = {
            let mut temp = None;
            for item in object.clone() {
                if item
                    .0
                    .iter()
                    .map(|i| i.get_symbol())
                    .collect::<Vec<String>>()
                    .join("\n")
                    == params
                        .iter()
                        .map(|i| i.get_symbol())
                        .collect::<Vec<String>>()
                        .join("\n")
                {
                    temp = Some(item.1);
                }
            }
            temp
        } {
            // Matched pattern
            let scope = scope.clone();
            eval_expr(program.to_string(), &scope)
        } else if let Some((args, (program, scope))) = {
            // Normal argument, not pattern
            let mut flag = None;
            for item in {
                let mut object = object.clone();
                object.reverse();
                object
            } {
                if item.0.iter().all(|i| matches!(i, Type::Symbol(_))) {
                    flag = Some(item);
                    break;
                }
            }
            flag
        } {
            let scope: &mut HashMap<String, Type> = &mut scope.clone();
            scope.extend(memory.to_owned()); // Update memory
            if args[args.len() - 1].get_symbol().starts_with("~") {
                for (arg, value) in args.iter().zip(params.to_vec()) {
                    // Processing of mutable length argument
                    if arg.get_symbol().starts_with("~") {
                        scope.insert(
                            arg.get_symbol()[1..arg.get_symbol().len()].to_string(),
                            Type::List(
                                params[params
                                    .iter()
                                    .position(|i| i.get_symbol() == value.get_symbol())
                                    .unwrap()..params.len()]
                                    .to_vec(),
                            ),
                        );
                    } else {
                        // Set argument value as variable
                        scope.insert(arg.get_symbol(), value);
                    }
                }
            } else {
                for (arg, value) in args.iter().zip(params.to_vec()) {
                    // Set argument value as variable
                    scope.insert(arg.get_symbol(), value);
                }
            }

            if args.len() <= params.len() {
                // Execute function code
                if let Type::Block(block) = Type::parse(program.clone()) {
                    run_program(block, scope)
                } else {
                    eval_expr(program.to_string(), scope)
                }
            } else {
                // Partial application of the function
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
    } else if let Function::Python(code, depent) = function {
        call_python(code, params, depent).unwrap_or(Type::Null)
    } else if let Function::Module(code) = function {
        let result = run_program(code, &mut memory.clone());
        if let Type::Function(func) = result {
            call_function(func, params, &memory.clone())
        } else {
            result.clone()
        }
    } else {
        return Type::Null;
    }
}

fn call_python(code: String, args: Vec<Type>, depent: Vec<String>) -> Option<Type> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let context = PyDict::new(py);
        let code = format!(
            "
{}
result = main({})
       ",
            code,
            args.iter()
                .map(|i| i.to_pyobj())
                .collect::<Vec<String>>()
                .join(", ")
        );

        for lib in depent {
            let module = if let Ok(module) = py.import(lib.as_str()) {
                module
            } else {
                return None;
            };
            let Ok(_) = context.set_item(lib, module) else {
                return None;
            };
        }

        if py.run(&code, Some(context), Some(context)).is_err() {
            return None;
        };
        let result = context.get_item("result").unwrap();
        Some(Type::from_python(result))
    })
}
