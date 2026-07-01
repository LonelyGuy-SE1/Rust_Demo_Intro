use std::fs;
fn main(){
let content="Hello Kirara!";

fs::write("message.txt", content).expect("Unable to write file.");

println!("File Written Successfully.Check using xdg-open message.txt .");
}
