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



fn main() {
    println!("Hello, world!");

    let m:u64 = 3820;
    let n:u64 = 32;
    let gretest=gcd(m, n);
    println!("{}", gretest);


    let t:u64 = m;
    {
        println!("evaluating cos x");
        t.cos()
    }

}
