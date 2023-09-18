// implicit - automatically (not supported)
// explicit - manually (supported)

fn main(){
    let x = 2; // by default i32

    println!("{} bytes",std::mem::size_of_val(&x));

    let y = 6i16;

    println!("{} bytes",std::mem::size_of_val(&y));

    let z = x as i16;

    println!("{} bytes",std::mem::size_of_val(&z));

}