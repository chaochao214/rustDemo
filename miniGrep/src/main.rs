use std::env;


fn main() {


    //cargo run needle haystack
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];

    println!("----------");
    println!("Searching for {}", query);
    println!("In file {}", filename);
    println!("{:?}", args);
}
