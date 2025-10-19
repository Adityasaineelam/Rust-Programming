//Error handling techniques
fn main(){
    //Approach one 
    enum option<T>{ //define the generic type
    Some(T), //Represents a value
    None, //Represents no value
    
}
//Approach 2
enum Result<T,E>{
    Ok(T) // represents a value 
    Err(E),
    
}
// Example on option<T>
enum MyOption<T> {
    Some(T),
    None,
}

// Function that returns an Option<f64>
fn divide(numerator: f64, denominator: f64) -> MyOption<f64> {
    if denominator == 0.0 {
        MyOption::None
    } else {
        MyOption::Some(numerator / denominator)
    }
}

fn main() {
    let result = divide(10.0, 2.0);
    match result {
        MyOption::Some(value) => println!("Result: {}", value),
        MyOption::None => println!("Cannot divide by zero"),
    }
}
//Approach 2
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}
// Approach 2
fn main() {
    let result = divide(1.0, 0.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Error! Division by zero."),
    }
}
    