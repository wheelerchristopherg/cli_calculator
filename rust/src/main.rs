use std::{collections::HashMap, env};

use cli_calculator::{evaluate_string_expression, main_loop};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        main_loop();
    } else if args.len() == 3 && args.get(1).unwrap_or(&"".to_owned()) == "--expression" {
        let default = "".to_owned();
        let e = args.get(2).unwrap_or(&default);
        let mut env = HashMap::new();
        let mut output = String::new();
        let mut index = 0;
        for expression in e.split(';') {
            match evaluate_string_expression(expression, &mut env, index) {
                Ok(value) => {
                    output.push_str(&value.to_string());
                    index += 1;
                }
                Err(e) => output.push_str(&e.to_string()),
            };
            output.push('\n');
        }
        print!("{}", output);
    } else {
        println!("invalid arguments");
    }
}
