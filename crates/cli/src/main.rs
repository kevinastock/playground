use std::env;

fn main() {
    let name = env::args().nth(1).unwrap_or_else(|| "world".to_string());
    println!("{}", client::greeting(&name));
}
