use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let fp = &args[2];

    println!("Searching for {query}");
    println!("In file {fp}");

    let contents = fs::read_to_string(fp).expect("Should be able to read the file");
    println!("In file contents {contents}");
}
