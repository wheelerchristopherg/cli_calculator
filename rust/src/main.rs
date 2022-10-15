use std::env;

use cli_calculator::{main_loop, parse_expression};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        main_loop();
    } else if args.len() == 3 && args.get(1).unwrap_or(&"".to_owned()) == "--expression" {
        let default = "".to_owned();
        let e = args.get(2).unwrap_or(&default);
        parse_expression(e);
    } else {
        println!("invalid arguments");
    }
}
