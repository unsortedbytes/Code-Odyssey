fn main() {
    println!("Hello, world!");


    open_store("DN nagar");
    // bake_pizza(true, "pepperoni");
    bake_pizza(15, "pepperoni");
    swin_in_profit();
    swin_in_profit();
    swin_in_profit();

    println!("The square funtion return {}", square(4));

    // unit -> tuple without any value
    let result = (); //-> () type
    // the default value return type of the funtion

    let result = mystry();

    // Block {}
    let multipler = 4;

    let calculation = {
        let value :i32 = 5+4;
        value*multipler
    };

    println!("The calculation is {calculation}");

}

fn mystry(){}

fn square(number:i32) -> i32{
    // return number*number;
    number*number
}

fn open_store(neighborhood:&str){
    println!("Opening my pizza store in {neighborhood}");
}

fn bake_pizza(number :i32, topping:&str){
    println!("Baking {number} {topping} pizza ");
}

fn swin_in_profit(){
    println!("So much $$$, so little  time");
}
