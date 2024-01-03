// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to subtract two numbers 
// * Use a function to multiply two numbers 
// * Use a function to subtract two numbers 
// * Use a function to subtract two numbers 
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result


// * Use a function to add two numbers together
fn add_two_numbers(a:i32,b:i32) -> i32{
    a + b
}

// * Use a function to subtract two numbers 
fn subtract_two_numbers(a:i32,b:i32) -> i32{
    a - b
}

// * Use a function to multiply two numbers 
fn multiply_two_numbers(a:i32,b:i32) -> i32{
    a * b
}

// * Use a function to subtract two numbers 
fn divide_two_numbers(a:i32,b:i32) -> i32{
    a / b
}

// * Use a function to subtract two numbers 
fn modulus_two_numbers(a:i32,b:i32) -> i32{
    a % b
}

// * Use a function to display the result
fn display_result(result: i32, operation:&str){
    // * Use the "{:?}" token in the println macro to display the result
    println!("The {:?} operation result is {:?}", operation, result);
}
fn main() {
    let sum: i32= add_two_numbers(2,3);
    let difference: i32= subtract_two_numbers(2,3);
    let product: i32= multiply_two_numbers(2,3);
    let division: i32= divide_two_numbers(2,3);
    let modulus: i32= modulus_two_numbers(2,3);

    display_result(sum,"addition");
    display_result(difference, "subtraction");
    display_result(product, "multiplication");
    display_result(division, "division");
    display_result(modulus, "modulus");
}


