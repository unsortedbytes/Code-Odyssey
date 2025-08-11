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
}
