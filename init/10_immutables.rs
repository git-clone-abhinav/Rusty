fn main(){
    let age = 16;
    println!("Age is {}", age);
    age = 17; // produces error, since variables are immutable by default
    println!("Age is {}", age);
}