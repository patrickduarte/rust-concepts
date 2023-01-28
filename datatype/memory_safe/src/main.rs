use std::io;

fn main() {
    
    /*
    Try to access index up to 4. Above 4, you will get runtime error, because  5 index don't exist.
    This is an example of Rust’s memory safety principles in action. In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing. Chapter 9 discusses more of Rust’s error handling and how you can write readable, safe code that neither panics nor allows invalid memory access.
     */
    let a = [10,27,89,16,16];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
        
  let index: usize = index 
        .trim()  
        .parse()
        .expect("Index entered was not a number");

  let element = a[index];

  println!("The value of the element at {index} is {element}");


}

