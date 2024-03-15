/* mut */
//varables in rust are immutable by default but you can add mut to change the mutability
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

/*constants */
// similar to solidity, it cannot be changed and the naming convention is in caps
//const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

/*shadowing */
//reusing of variable names

fn main_() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("the value of x in the inner scope is : {x}");
    }
    println!("the value of x is: {x}");
}

//data types
//let guess: u32 = "42".parse().expect("Not a number!");
//use parse to convert string to numbers
/*
scalar and compound data type subset
 scalar rep a single value
 >integers = no fraction, u32(unsigned + ) , i32(signed + or -)
 >floating-point numbers
 >Booleans
 >characters
 
 
 
 
 */


