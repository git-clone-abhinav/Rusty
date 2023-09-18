fn main(){
    let name = "Charmy";
    let name_1 = "Abhinav";
    println!("{} - {}",name, name_1);

    let _name = "Hii";
    println!("{}", _name);

    let a = 50;
    let A = 60; // generates error as variable names are case sensitive, but still the varaibles get created
    println!("{} - {}", a, A);

    // keywords in Rust : https://doc.rust-lang.org/reference/keywords.html
}