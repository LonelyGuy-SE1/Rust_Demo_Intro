struct Student {
    name: String,
    age: u8,
}

impl Student {
    fn new(name: String, age: u8) -> Self {
        Student { name, age }
    }

    fn display(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
}

fn main() {
    let student = Student::new(String::from("Alice"), 19);
    student.display();
}   