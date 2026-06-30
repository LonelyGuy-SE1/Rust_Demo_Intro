struct Student{
name: String,
age: u32,
}

impl Student{
fn display(&self){
println!("Name : {}",self.name);
println!("Age : {}",self.age);
}
}
fn main(){
let student = Student{
name: String::from("Alice"),
age: 19,
};
student.display();
}

