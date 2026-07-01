use std::fs;

fn main(){
let content=fs::read_to_string("message.txt").expect("Unable to read.");

println!("{}",content);
}
