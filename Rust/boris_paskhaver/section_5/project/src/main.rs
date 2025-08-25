fn main() {
    println!("Hello, world!");
    println!("\n\n\n\n\n\n");
    println!("red color to no is {}",color_to_number("red"));
    println!("green color to no is {}", color_to_number("green"));
    println!("blue color to no is {}", color_to_number("blue"));
    println!("greeno color to no is {}", color_to_number("greeno"));

    println!("\n\n\n\n\n\n");
    println!("red color to no is {}",color_to_number_match("red"));
    println!("green color to no is {}", color_to_number_match("green"));
    println!("blue color to no is {}", color_to_number_match("blue"));
    println!("greeno color to no is {}", color_to_number_match("greeno"));

    println!("The factorial of 5 is {}", factorial(5));
    println!("The factorial of 6 is {}", factorial(6));

    println!("The factorial of 5 is {}", recorsion_factorial(5));
    println!("The factorial of 6 is {}", recorsion_factorial(6));
}

fn recorsion_factorial(mut n:i32) -> i32{
    if n ==1 || n == 0 {
        return 1;
    }
    return n * recorsion_factorial(n-1);
}

fn factorial(mut n:i32) -> i32 {
    let mut fact:i32 = 1;
    if n == 1 || n == 0 {
        return 1;
    }
    while  n>=1 {
        fact = fact * n;
        n-=1;
    }
    return fact;
}

fn color_to_number_match(color:&str)-> i32{
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _=> 0
    }
}



fn color_to_number(color :&str) ->i32{
    if color == "red" {
        return 1;
    }else if color == "green" {
        return 2;
    }else if color =="blue" {
        return 3;
    }
    0
}

