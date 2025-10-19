// Looping
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("Count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            remaining -= 1;
        }

        if count == 2 {
            break 'counting_up;
        }
        count += 1;
    }
}
// while loop
fn main() {
    let mut count = 0;
    while count < 10 {
        println!("Count: {}", count);
        count += 1;
    }
}
// while loop
fn main(){
let mut number=3;
while number!=0{
    println!("{number}");
    number-=1;
    break;
}
println!("Hey");
}
// looping through a collection with for loop
fn main() {
    let a = [1, 2, 3, 4, 5, 6];
    let b = ["a", "b", "c", "d"];

    for element in a {
        println!("{}", element);
    }

    for element in b {
        println!("{}", element);
    }
}