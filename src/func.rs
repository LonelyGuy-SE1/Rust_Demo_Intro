fn area(length: f64, width: f64)->f64{
length*width
}

fn main(){
let mut result=area(5.5,4.2);
result+=0.9;
println!("Area = {}", result);
}
