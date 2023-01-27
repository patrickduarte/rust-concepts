fn main() {

let guess: u32 = "42".parse().expect("Not a number!");

   println!("O valor de guess é {guess}");

   //First primitive type for floating-point numbers
   let x = 2.0; // f64 is default

   let y: f32 = 3.0; // f32

   println!("x is {x}");
   println!("y is {y}");

   //Second primitive type for floating-point numbers

       // addition
       let sum = 5 + 10;
       println!("sum is {sum}");
       // subtraction
       let difference = 95.5 - 4.3;
       println!("difference is {difference}");
   
       // multiplication
       let product = 4 * 30;
       println!("product is {product}");
   
       // division
       let quotient = 56.7 / 32.2;
       println!("quotient is {quotient}");
       
       //less precision
       let quotient32: f32 = 56.7 / 32.2;
       println!("quotient32 is {quotient32}");

       let truncated = -5 / 3; // Results in -1
       println!("truncated is {truncated}");
   
       // remainder
       let remainder = 43 % 5;
       println!("remainder is {remainder}");

       //All rust operations can consulted here
       // https://doc.rust-lang.org/book/appendix-02-operators.html

       // Boolean types

       let t = true;
       let f: bool = false; // with explicit type annotation

       println!("t is {t}");
       println!("f is {f}");

       // The character type
       //Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes

       let c = 'z';
       let z: char = 'ℤ'; // with explicit type annotation
       //Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust
       let heart_eyed_cat = '😻';

       println!("c is {c}");
       println!("z is {z}");
       println!("heart_eyed_cat is {heart_eyed_cat}");
 


}
