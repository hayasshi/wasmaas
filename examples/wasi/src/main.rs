use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default = String::from("world");
    let name = args.get(1).unwrap_or(&default);
    println!("Hello, {}!", name);
}
