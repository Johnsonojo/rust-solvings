// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person{
    name: String,
    age: i32,
    fav_color: String
}

impl Person {
    fn print_name_and_color(&self){
        println!("The person's name is {:?}, and their favorite color is {:?}", self.name, self.fav_color)
    }
}
fn main() {
    // * Create and store at least 3 people in a vector
    let mut persons: Vec<Person> = Vec::new();

    persons.push(Person{age:10,name: "Dara".to_owned(),fav_color: String::from("Black"),});
    persons.push(Person{age:11,name: "Lara".to_owned(),fav_color: String::from("Brown"),});
    persons.push(Person{age:12,name: "Ogaga".to_owned(),fav_color: String::from("Blue"),});
    persons.push(Person{age:9,name: "Mary".to_owned(),fav_color: String::from("Green"),});
    persons.push(Person{age:8,name: "Sewa".to_owned(),fav_color: String::from("Red"),});

    // * Iterate through the vector using a for..in loop
    for person in persons {
        // * Use an if expression to determine which person's info should be printed
        if person.age <= 10 {
            // * The name and colors should be printed using a function
            person.print_name_and_color();
        }
    }
}
