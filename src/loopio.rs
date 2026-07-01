use std::io;

fn main(){
let mut input=String::new();

println!("Enter a number : ");

io::stdin().read_line(&mut input).expect("Failed to load number.");

let number: i32=input.trim().parse().unwrap();

let mut sum=0;

for i in 1..=number{

sum+=i;}

println!("Sum is {}!", sum);
}
