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
        match evaluate_string_expression(e, &mut env, 0) {
            Ok(value) => println!("{}", value),
            Err(e) => println!("{}", e),
        }
    } else {
        println!("invalid arguments");
    }
}
