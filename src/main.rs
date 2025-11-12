use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let fp = &args[2];

    println!("Searching for {query}");
    println!("In file {fp}");
}
