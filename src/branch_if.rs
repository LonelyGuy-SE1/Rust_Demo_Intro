use std::io;

fn main(){
let mut input=String::new();
println!("Enter volatge:");
io::stdin().read_line(&mut input).unwrap();
let v: i32=input.trim().parse().unwrap();
if v>40{
println!("High Voltage");}else{println!("Low Voltage");}
}
