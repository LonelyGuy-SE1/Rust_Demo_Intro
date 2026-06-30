struct BoxValue<T>{
value : T}

impl<T>BoxValue<T>{

fn get_value(&self)->&T{&self.value}}

fn main(){

let box1=BoxValue {value:42};println!("{}",box1.get_value());}

