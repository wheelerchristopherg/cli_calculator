mod tokens;
use tokens::Token;

fn main() {
    let s = "    \t";
    let t: Token = Token::from(s);
    println!("{} = {:?}", s, t);
}
