fn main(){
    let mut age = 18;
    const NAME: &str = "Charmy"; // const requires type annotation & all caps
    println!("Hello {} your age is {}", NAME, age);

    // This will not throw error
    age = 20;
    println!("Hello {} your age is {}", NAME, age);

    // This will throw error
    NAME = "Charmy";
    println!("Hello {} your age is {}", NAME, age);

    // This will throw error also if use "mut" keyword used on constants.
}