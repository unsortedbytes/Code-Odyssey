const TOUCHDOWN_POINTS:i32 = 6;

fn main() {
    let season:&str = "Summer";

    let points_scored:i32 = 10;

    let mut points_scored :i32 = 28;
    points_scored = 35;


    let event_time = "06:00";
    let event_time = 6;

    println!("This {season} the points scored is {points_scored} in time of {event_time} with touchdown points {TOUCHDOWN_POINTS}");
    println!("This {} the points scored is {} in time of {} with touchdown points {}", season, points_scored, event_time, TOUCHDOWN_POINTS);
    println!("This {1} the points scored is {3} in time of {2} with touchdown points {0}",TOUCHDOWN_POINTS, points_scored, event_time, points_scored);


    //let _favorite_beverage = "Sprite"
    #[allow(unused_variables)]
    let favorite_beverage = "Maza";
    
}
