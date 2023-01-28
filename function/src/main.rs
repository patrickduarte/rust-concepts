fn main() {
    println!("Hello, world!");

    another_function();
    another_function2(5);
    print_labeled_measurement(5, 'h');

    /*
     - Statements are instructions that perform some action and do not return a value.
     - Expressions evaluate to a resultant value. Let’s look at some examples.
     
     let y = 6;  Statements example
     let x = (let y = 6);  This is don't allowed. It get a error, because "let y = 6" is a statement that don't return a value

    */
     //Below, expression example.
    
     let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    //Functions with Return Values

    let xx = five();

    println!("The value of xx is: {xx}");

    // Second example with parameters
    let x = plus_one(5);

    println!("The value of x is: {x}");



}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}


fn plus_one(x: i32) -> i32 {
    x + 1 // Don't use semicolon ";"  because changing it from expression to a statement
}