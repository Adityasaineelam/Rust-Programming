// Ownership
fn main(){
    let s1=String::from("Rust");
    let len=calculate_length(&s1);
    println!("Length of '{}' is {}.",s1,len);
}
fn calculate_length(s:&String)->usize{
    s.len()
}

// only one owner at a time
fn main(){
    let s1=String::from("RUST");
    let s2=s1;
    println!("{}",s2);
}

// when the owner goes out of scope the value will be dropped 
fn main(){
    let s1=String::from("RUST");
    let len=calculate_length(&s1);
    println!("Length of '{}' is {}.",s1,len);
}
// s1 goes out of scope and gets dropped 
fn calculate_length(s:&String)->usize{
    s.len()
}

