use reqwest::blocking;

fn main(){
let response=blocking::get("https://httpbin.org/get").expect("Request Failed.");
let body=response.text().expect("Failed to read response");

println!("{}", body);
}
