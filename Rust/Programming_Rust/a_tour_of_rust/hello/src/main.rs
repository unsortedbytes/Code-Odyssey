fn gcd(mut n:u64, mut m:u64) -> u64{
    assert!(n!=0  && m!=0);
    while m!=0{
        if m<n{
            let t=m;
            m=n;
            n=t;
        }
        m = m%n;
    }
    n

}

// Testing 
#[test]
fn test_gcd(){
    assert_eq!(gcd(13, 25),2);
    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}



fn main() {
    println!("Hello, world!");

    let m:u64 = 3820;
    let n:u64 = 32;
    let gretest=gcd(m, n);
    println!("{}", gretest);


    let t:u64 = m;
    let cosoft={
        println!("evaluating cos x = {}", (t as f64).cos());
        // (t as f64).cos();
        (t as f64).cos()
    };
    
    println!("The valueof cosoft is {}", cosoft);
}
