struct BoxValue<T>{
value: T,
}

fn main(){

let int_box=BoxValue{value:10};
let float_box=BoxValue{value: 3.14};
let text_box=BoxValue{value: String::from("Hello"),};

println!("{}",int_box.value);
println!("{}",float_box.value);
println!("{}",text_box.value);
}
