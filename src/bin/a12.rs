// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics


// * Use an enum for the box color
enum BoxColor{
    White,
    Black,
    Brown
}

impl BoxColor{
    // * Implement functionality on the box struct to print the box color
    fn print_color(&self){
        match self{
            BoxColor::White => println!("The box color is white"),
            BoxColor::Black => println!("The box color is black"),
            BoxColor::Brown => println!("The box color is brown"),
        }
    }
}

// create a struct to encapsulate the box dimensions
struct BoxDimension{
    length:f64,
    width:f64,
    height:f64,
}

impl BoxDimension{
    fn print_dimension(&self){
        println!("The box dimension is {:?} length by {:?} width by {:?} height ", self.length, self.width, self.height);
    }
}
// * Use a struct to encapsulate the box characteristics
struct ShippingBox{
    dimension:BoxDimension,
    weight:f64,
    color:BoxColor,
}

// * Implement functionality on the box struct to create a new box
impl ShippingBox {
    fn new(dimension:BoxDimension, weight:f64, color:BoxColor) -> Self{
        Self{
            dimension,
            weight,
            color
        }
    }

    // * Implement functionality on the box struct to print the characteristics
    fn print_char(&self){
      self.dimension.print_dimension();
      self.color.print_color();
      println!("The box weight is {:?} kg", self.weight);
}
}

fn main() {
    let small_dimension = BoxDimension{length:10.0, width:20.0, height:30.0};
    let white_box = ShippingBox::new(small_dimension, 20.0, BoxColor::White);
    white_box.print_char();   

    let medium_dimension = BoxDimension{length:15.0, width:25.0, height:35.0};
    let brown_box = ShippingBox::new(medium_dimension, 15.0, BoxColor::Brown);
    brown_box.print_char();

    let large_dimension = BoxDimension{length:20.0, width:30.0, height:40.0};
    let black_box = ShippingBox::new(large_dimension, 10.0, BoxColor::Black);
    black_box.print_char();
}

