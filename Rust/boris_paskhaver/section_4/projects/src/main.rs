fn main() {
    println!("Hello, world!");
    apply_to_job(35, "Rust Developers");
    println!("Is 4 is even {}", is_even(4));
    println!("Is 5 is even {}", is_even(5));
    println!("{:?}", alphabets("aardvark"));// -> (true, false)
    println!("{:?}", alphabets("zoology"));//  -> (false, true)
    println!("{:?}", alphabets("zebra")); //   -> (true, true)
}

fn alphabets(text :&str)-> (bool, bool) {
    let mut result :(bool, bool)= (false, false);
    for char in text.chars() {
        if char == 'a' {
            result.0 = true;
        }
        if char == 'z'{
            result.1 = true;
        }
    }

    result
}

fn is_even(number :i32 ) -> bool {
    if number%2 == 0 {
        return true;
    }
    false
}

fn apply_to_job(number:i32, title:&str) {
    println!("I'm applying to {number} {title} jobs");
}
