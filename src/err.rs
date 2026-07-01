use std::io;

fn main(){
let mut input=String::new();

println!("Enter a number : ");

io::stdin().read_line(&mut input).expect("Failed to read input.");

match input.trim().parse::<i32>(){
    Ok(number)=>println!("You entered {}.", number),
    Err(_)=>println!("Invalid integer entered."),
}
}
