// compound data types
// arrays,tuples,slices and strings
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Numbers Array: {:?}", numbers);
    let mix=[1,2,"apple",true];
    println!("Mix array:{:?}",mix);
    let fruits: [&str;3]=["Apple","Banana","Orange"];
    println!("Fruits Array: {:?}",fruits);
    println!("First element:{}",fruits[0]);
    println!("Second element:{}",fruits[1]);
    println!("Third element:{}",fruits[2]);
}
//tuples
fn main(){
   let human: (String,i32,bool)=("Alice".to_string(),30,false);
    println!("Human tuple:{:?}",human);
    let my_mix_tuple=("kratos",23,true,[1,2,3,4,5]);
    println!("My mix tuple:{:?}",my_mix_tuple);
}

// Slices 
fn main(){
    let number_Slices:&[i32]=&[1,2,3,4,5];
    println!("Number Slice:{:?}",number_Slices);
}
 fn main(){
    let book_slices:&[&String]=&[&"IT".to_string(),&"Harry Potter".to_string(),&"ZEN".to_string()];
     println!("Book Slice:{:?}",book_slices);
 }
 fn main(){
 let stone_cold: String=String::from("Hell, ");
 println!("Stone Cold says: {}",stone_cold);
 }
 fn main(){
 let mut stone_cold: String=String::from("Hell, ");
 println!("Stone Cold says: {}",stone_cold);
 stone_cold.push_str("Yeah!");
 }
 //B- & str(String slice)
 fn main() {
    let string: String = String::from("Hello, world");
    let slice: &str = &string[0..5]; // or string.as_str()
    println!("Slice value: {}", slice);
}


