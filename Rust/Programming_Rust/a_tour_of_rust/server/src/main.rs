extern crate iron;
// #[macro_use] extern crate mime;

use iron::mime::Mime;
use iron::prelude::*;
use iron::status;


fn main() {
    println!("Hello, world!");
    println!("Serving on http://localhost:3000...");
    Iron::new(get_form).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response>{
    let mut response = Response::new();

    response.set_mut(status::Ok);
    // response.set_mut(mime!(Text/Html; Charset=Utf8));
    // response.set_mut(mime(Text/Html));
    response.set_mut("text/html".parse::<Mime>().unwrap());
    response.set_mut(r#"
        <title>GCD Calculator</title>
        <form action="/gcd" methods="post">
            <input type="text" name="n" />
            <input type="text" name="" />
            <button type="submit">Compute GCD</button>
        </form>
    "#);
    Ok(response)
}
