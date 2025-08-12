type Meters = i32;

fn main() {
    //println!("Hello, world!");
    let apples = 50;
    let oranges = 14 +8;
    let _fruits = apples + oranges;

    println!("My Garden ");
    println!("My house");
    // {} -> intrapolate
    println!("This year, my garden have {} apples.", apples -10);
    //println!("This year, my garden have {apples  - 10} apples");
    // Case study 
    println!("This year, my garden have {} apples and {} oranges", apples, oranges);

    println!("This year, my garden have {1} apples and {0} oranges total {1} fruits", apples, oranges);
    // This help to unrepeate the arugments 

    // Immutable 
    

    let gym_reps = 10;
    let mut gym_reps_mut = 20;
    println!("I plan to do {gym_reps} reps");

    //gym_reps = 15;
    gym_reps_mut = 23;
    println!("############################################################################");
    println!("############################################################################");
    println!("Variable shawdoing");
    let grams_of_protein ="100.345";//->&str
    
    //
    let grams_of_protein = 100.345;//-> f64
    //
    let mut grams_of_protein = 100;

    grams_of_protein = 98;


    println!("#################################################################################");
    println!("#***************** Scopes *******************************************************");
    println!("##################################################################################");

    let coffee_price = 5.99;
    
    {
        let cookie_price = 1.99;
        let coffee_price = 43;// not a variable shawdoing 
        println!("The Coffee_price is {coffee_price}");
        println!("The cookie price is {cookie_price}");

    }
    println!("The coffee price is {coffee_price}");


    println!("##################################################################################");
    println!("#************* Constansts *******************************************************#");
    println!("########;###########################################################################");

    //println!("The value of new_cat is {TEST}");

    const NEW_CAT:i32 = 43;
    {
        const TEST :f32 = 24.22;
    }
    println!("The value of Test is {TEST}");
    println!("The value of new_cat is {NEW_CAT}");
    



    println!("#####################################################################################");
    println!("# *             Types Alies              * #");
    println!("###################################################################################");

    
    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = 3210;
    println!("A  one  mile race is {mile_race_length} meters long and a two mile is {two_mile_race_length}");




    println!("###############################################################################");
    println!("# *               Compiler Directives                    * #");
    println!("###############################################################################");

    #[allow(unused_variables)]
    let mike = 1600;
    #[allow(unused_variables)]
    let tow = 3200;
}

const TEST:f64 = 923.43;
