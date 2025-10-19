//Comments
//This is one line Comment 
fn main(){
    //this is for printing hello world
    println!("Hello,World");
    //println!("Hello,World");
    println!("I feel lucky");
}
// Control flow - IF ELSE CONDITIONS
// Control flow in rust
//1-Conditions [IF...ELSE]
//2-Repeating actions[LOOPS]
fn main(){
    let x=5;
    let y=10;
    if x>y{
        println!("x is greater than y");
    }else{
        println!("y is greater than x");
    }
}
// age
fn main(){
    let age:u16=18;
    if age>=18{
        println!("You are allowed to vote:");
    }
    else{
        println!("You are  not allowed to vote:");
    }
}
//Multiple conditions with else if:
fn main(){
    let number=6;
    if number %4==0{
        println!("number is divisible by 4");
    }
    else if number %3==0{
        println!("number is divisible by 3");
    }
    else if number%2==0{
        println!("number is divisible by 2");
    }
    else{
        println!("number is not divisible by 2,3,4");
    }
}
// Using let in if statement
fn main(){
    let condition=true;
let number if condition {5} else {6};
println!("Number:{number}");
}

