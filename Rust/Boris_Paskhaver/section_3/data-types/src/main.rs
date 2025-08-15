#![allow(unused_variables)]
fn main() {
    //println!("Hello, world!");
    
    let f1 : f64 = 0.1;
    let f2 : f64 = 0.3;
    let sum: f64= f1+ f2;


    println!( "{}",sum==0.3);

    println!("#*************************************#");
    println!("#            Integers                #");
    println!("#**************************************#");

    //let eight_digit:i8 = -210;
    let eight_digit_unsigned:u8 = 234;

    let digit : i128 = 92749298339872838798839238792802843219;
    println!("{digit}");

    println!("{}", digit-845797507092874587858);

    let some=20u16;

    // using _ as a visual_separator_for_numbers;
    println!("{}", 2039==2_0_3_9);

    let days:usize = 29;
    let years:isize = 329;


    println!("##############################################################");
    println!("#         Srings                                    #");
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");

    // Stirng liters -> string which know at compile time 
    println!("Dear Aditya,\n How have you been?");
    println!("\t Ones upon a time");
    println!("ones is said \"ok \" ");
    println!("File Path C:\\My Documemts\\New file\\ok.txt");

    // Raw stirng 
    println!(r"File Path C:\My Documents\New file\Ok.txt");



    println!("%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%");
    println!("#********                     Methods             ***********#");
    println!("%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%");

    let value :i32 = -15;
    println!("{}", value.abs());
    
    let empty_space = "       MY conent               ";
    println!("{}", empty_space );
    println!("{}", empty_space.trim());
    println!("{}", value.pow(2));


    let pi:f64 = 3.1415926535897932384;
    println!("The value of Pi is {pi}");

    println!("The floor of PI is {}", pi.floor());
    println!("The celing of PI is {}", pi.ceil());
    println!("The round of PI is {}", pi.round());

    println!("%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%");
    println!("#*********             Formate specifier              *******************#");
    println!("%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%");

    let PI:f64 = 3.141592653589793238729874972879799832975428498284993849798273847892792797498728748723874882348782784773874872878482388034983948982983493456789093456789087654345678908765435678987654356789098765467890098765434567897654356789876548765434567890987654345678987654345678976543456789893874;
    println!("The value using formate specifier is {pi:.128}");



    println!("\n\n\n%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%");
    println!("\n \n Type casting \n\n");

    let miles_away=10;
    let miles_away_i8 = miles_away as i8;
    let miles_away_i16 = miles_away as i16;

    let miles_away_f64 = miles_away as f64;
    



    println!("\n\n\n %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%");
    println!("\n\n\n Augmented Assigment Operator  \n");

    let mut year:i32 = 2025;
    year +=1;
    println!("The new year is {}",year);
    year -=5;
    println!("The new year is {}",year);
    year *=2;
    println!("The new year is {}",year);
    year /= 4;

    println!("The new year is {}", year);











    println!("\n\n\n\n\n\n\n");
    

    let is_handsome:bool = true;
    let is_silly:bool = false;

    println!("Handsome:{is_handsome}. Silly: {is_silly}");
    let age:i32 =  21 ;
    let is_young:bool = age < 35;
    println!("{is_young}");
    println!("{} {}", age.is_positive(), age.is_negative());


    // !-> invert 
    println!("{}", true);
    println!("{}", !true);
    println!("{}", !false);


    let age = 13;
    let can_see_rated_r_movie = age >= 17;

    let cannot_see_rated_r_moive:bool = !can_see_rated_r_movie;

    println!("I am {age} year old. Can I see this scarry movie? {can_see_rated_r_movie}");

    println!("{}" , "Cooke" == "cooke");
    println!("{}", "Coke" != "coke");

    println!("{}", 13==13.01 as i32);


    println!("\n\n\n\n\n\n\n\n\n\n\n\n");
    let purchased_ticket = true;
    let plane_on_time = false;
    let making_event = purchased_ticket && plane_on_time;
    println!("It is {} that I will arrive as expected", making_event);

    println!("\n\n\n\n");
    println!("{}", true || false);

    let user_has_paid_for_subscription = true;
    let user_is_admin = true;
    let user_can_see_premium_experience = user_is_admin || user_has_paid_for_subscription;
    println!("Cann this user see my site {}", user_can_see_premium_experience);


    println!("\n\n\n\n\n");
    let first_initial = 'B';//-> char
    let example = "B";// -> stirng literals
    let emoji:char = '😄';
    println!("{emoji}");
    println!("{} {}", emoji, emoji.is_alphabetic());
    println!("{} {}", first_initial, first_initial.is_alphabetic());
    println!("{} {}", first_initial, first_initial.is_uppercase());
    println!("{} {}", first_initial, first_initial.is_lowercase());


    println!("\n\n\n\n\n");

    let numbers:[i32;6] = [4, 5, 3, 1, 53, 2];

    let apples = ["Aditya", "kumar", "21MF10004"];
    println!("Length: {}", apples.len());

    let currency_rates:[i32;3] ;

    let seasons:[&str; 4] = ["Spring", "Summer", "Fall", "Winter"];

    let second = seasons[1];
    let first = seasons[0];
    let third = seasons[2];
    let fouth = seasons[3];
    // let fith = seasons[4];
    println!("The first season is {first} and the second season is {second}");


}
