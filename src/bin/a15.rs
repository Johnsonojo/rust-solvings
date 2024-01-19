// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info


// * Use an enum for the tickets with data associated with each variant
enum Ticket{
    Backstage(String,f64), 
    Standard(f64),
    Vip(String,f64)
}

// * Use a match expression while iterating the vector to print the ticket info
fn main(){
    // * Create one of each ticket and place into a vector
    let tickets = vec![
        Ticket::Backstage("backstage".to_owned(), 40.6),
        Ticket::Standard(30.6),
        Ticket::Vip("vip".to_owned(), 100.6),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(holder ,price)=> println!("Holder: {:?} and price: {:?}", holder, price),
            Ticket::Standard(price)=> println!("Price: {:?}", price),
            Ticket::Vip(holder ,price)=> println!("Holder: {:?} and price: {:?}", holder, price)
        }
    }
}