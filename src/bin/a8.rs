// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor


// * Use an enum to create different flavors of drinks
enum DrinkFlavor {
    Sparkling,
    Sweet,
    Fruity,
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: DrinkFlavor,
    fluid_ounces: f64,
}

// * Use a function to print out the drink flavor and ounces
fn print_drink(drink: Drink) {
    // * Use a match expression to print the drink flavor
    match drink.flavor {
        DrinkFlavor::Sparkling => println!("flavor: sparkling"),
        DrinkFlavor::Sweet => println!("flavor: sweet"),
        DrinkFlavor::Fruity => println!("flavor: fruity"),
    }

    println!("oz: {:?}", drink.fluid_ounces);
}
fn main() {
    let sparkling_drink = Drink {
        flavor: DrinkFlavor::Sparkling,
        fluid_ounces: 10.0,
    };
    let sweet_drink = Drink {
        flavor: DrinkFlavor::Sweet,
        fluid_ounces: 11.0,
    };
    let fruity_drink = Drink {
        flavor: DrinkFlavor::Fruity,
        fluid_ounces: 12.0,
    };
    print_drink(sparkling_drink);
    print_drink(sweet_drink);
    print_drink(fruity_drink);
}
