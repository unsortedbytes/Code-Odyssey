fn main() {
    println!("Hello, world!");
    // Ownership -> properties of comiler 
    println!("\n\n\n Ownership ");
    println!("Many Problem best of all");
    println!("\n\n\n");
    

    let age = 33;

    {
        let is_handsome = true;//-> stack memory
    }
    // println!("{}", is_handsome); -> error -> out of scope

    // age variable exists here
    





    println!("\n\n\n\n");

    // Copy trait 
    // Scalar type
    let time = 2025;
    let year = time;

    println!("The time is {time}. It is the year {year}");




    let mut food = "Aditya";
    println!("The food variable {food}");
    food = "songshfhh";
    println!("The food variable {food}");
    println!(r"&str is driectly convert into binary executable not in stack or heap memory");


    // String 
    let new_string :String= String::new();

    let from_string :String = String::from("Kitkat");
}// age variable goes out of scope here 
