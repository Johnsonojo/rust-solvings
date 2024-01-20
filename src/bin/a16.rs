// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
struct Student{
    name: String,
    locker: Option<i32>
}


fn main() {
    let new_student = Student{
        name: "becky".to_owned(),
        locker: Some(32)
    };

    println!("Student's name is {:?}", new_student.name);
    match new_student.locker {
        Some(locker) => println!("locker number is {:?}", locker),
        None => println!("no locker number")
    }
}
