struct Student{name: String, age:u32,}

impl Student{
fn new(name: String, age:u32)->Self{
Self{name, age}
}
}

impl Student{
fn greet(&self){println!("Hello, I am {}",self.name);}
}

impl Student{
fn birthday(&mut self){self.age+=1;}
}

fn main(){
let mut s=Student::new("Mika".to_string(),20);

s.greet();
s.birthday();

println!("{}",s.age);
}
