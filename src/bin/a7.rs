// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

// * Use an enum with color names as variants
enum Color  {
    Black,
    White,
    Green
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_color(color:Color){
    // * Use a match expression to determine which color
    match color {
        Color::Black => println!("black"),
        Color::White => println!("white"),
        Color::Green => println!("green"),
    };
}

fn main() {
    print_color(Color::Black);
    print_color(Color::White);
    print_color(Color::Green);
}
