struct Rectangle{
length: f64,width: f64,
}

impl Rectangle{
fn area(&self)->f64{self.length*self.width}
}

fn main(){
let room = Rectangle {
length:5.0,width:4.0,};

let table=Rectangle{
length:2.0, width:1.5,};

println!("Room Area = {}",room.area());
println!("Table Area = {}",table.area());
}
