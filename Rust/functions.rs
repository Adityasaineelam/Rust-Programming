//bmi calculation
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}

fn main() {
    let weight = 70.0; // in kilograms
    let height = 1.75; // in meters

    let bmi = calculate_bmi(weight, height);
    println!("Your BMI is {:.2}", bmi);
}

// basic addition
fn add(a: i32, b: i32) -> i32 {
    a + b
}

//area of circle

fn area_of_circle(radius: f64) -> f64 {
    std::f64::consts::PI * radius * radius
}

//reverse a string
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

//factorial
fn factorial(n: u32) -> u32 {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}


