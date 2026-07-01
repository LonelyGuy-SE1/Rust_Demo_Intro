use std::io;

mod fibonacci{
pub fn fibonacci(n: u32)->u32{
if n==0{
return 0;
}
if n==1{
return 1;
}
fibonacci(n-1)+fibonacci(n-2)
}
}

fn main(){
let mut input=String::new();

println!("Enter a number : ");

io::stdin().read_line(&mut input).expect("Failed to load number.");

let n: u32=input.trim().parse().unwrap();

let result = fibonacci::fibonacci(n);
println!("Fibonacci({}) is {}!", n, result);
}
