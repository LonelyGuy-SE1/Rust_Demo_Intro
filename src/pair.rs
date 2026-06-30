struct Pair<T,U>{
first:T,second:U,}

fn main(){
let p=Pair{first:100, second:"Rust",};
println!("{} {}", p.first, p.second);
}
