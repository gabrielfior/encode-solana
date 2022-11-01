use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();
    let first3args = &args[1..4];
    println!("{:?}", &first3args);
}
