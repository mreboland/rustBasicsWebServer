// extern allows the iron and mime crates that we cited in our Cargo.toml file available to our program
extern crate iron;
// The macro_use attribute alerts Rust that we plan to use macros exported by this crate.
#[macro_use]
extern crate mime;
// Importing and using (below) Router so that we can associate different handlers with different paths
extern crate router;

// The declaration below makes all the public names of the iron::prelude module directly visible in our code.
use iron::prelude::*;
// Generally it's preferable to spell out the name we wish to use as below, but by convention, when a module is named prelude, that means that its exports are intended to provide the sort of general facilities that any user of the crate will probably need. So in this case, using a wildcard * makes sense.
use iron::status;
use router::Router;

fn main() {
    // Creating a router
    let mut router = Router::new();

    // Establishing handler functions for two specific paths. We then pass router to Iron::new yielding a web server that consults the URL path to decide which handler function to call.
    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    // Printing a msg reminding us how to connect to our server
    println!("Serving on http://localhost:3000...");
    // Calling Iron::new to create a server, and then sets it listening on TCP port 3000
    // We pass the get_form function to Iron::new, indicating that the server should use that function to handle all requests.
    // Iron::new(get_form).http("localhost:3000").unwrap();
    Iron::new(router).http("localhost:3000").unwrap();
}

// The function takes a mutable reference, written &,ut, to a Request value representing the HTTP request we've been called to handle.
// The _ tells Rust that we expect the variable not to be used so it shouldn't warn us.
fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    // The set_mut method uses its argument's type to decide which part of the response to set. So each call to set_mut is actually setting a different part of response:
    // Passing status::Ok sets the HTTP status
    response.set_mut(status::Ok);
    // Passing the media type of the content (using the mime! macro) sets the Content-Type header
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    // Passing a string sets the response body
    // Since the response text contains a lot of double quotes, we use Rust's raw string syntax r#" "#. Any character may occur within a raw string without being escaped. If hashes appear in the string, which will causes closing issues, we add more hashes to the start and end to override it.
    response.set_mut(
        r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="n">
            <input type="text" name="n">
            <button type="submit">Compute GCD</button>
        </form>
    "#,
    );

    // Our function's return type, IronResult<Response> (at the top of function), is another variant of the Result type we encountered earlier (Ok(r) or Err(e)). We construct our return value as Ok(response) using the "last expression" syntax to implicitly specify the function's return value (of semi-colon)
    Ok(response)

    // We use cargo run which will fetch the needed crates, compile them, building our program, linking everything, and starting it up.
    // If everything is ok, we can go to http://localhost:3000/gcd
}

extern crate urlencoded;

use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    // The below match follows the Ok(s), Err(e) methodology. It's conditional, like an if statement. If it's Ok, run the Ok code, if not, run the Err code.
    // The request.get_ref... is called to parse the request's body as a table, mapping query parameter names to arrays of values. If it fails we report it under Err(e). The <UrlEnc...> part of the method call is a type parameter indicating which part of the Request get_red should retrieve. In this case it refers to the body, parsed as a URLencoded query string.
    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }

        Ok(map) => map,
    };

    // Within that table (created via UrlEncodedBody above), it finds the value of parameter names "n", which is where the HTML form places the numbers entered into the web page. This value will not be a single string but a vector of strings, as query parameter names can be repeated.
    let unparsed_numbers = match form_data.get("n") {
        // It walks (?) the vector of strings, parsing each one as an unsigned 64-bit number, and returning an appropriate failure page if any of the strings fail to parse.
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => nums,
    };

    let mut numbers = Vec::new();

    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!(
                    "Value for 'n' parameter not a number: {:?}\n",
                    unparsed
                ));
                return Ok(response);
            }

            Ok(n) => {
                numbers.push(n);
            }
        }
    }

    // We compute the numbers' greatest common divisor
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    // We construct a response describing the results. The format! macro uses the same kind of string template as the writeLn! and printLn! macros, but returns a string value, rather than writing the text to a stream.
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        format!("The greatest common divisor of the numbers {:?} is <b>{}</b>\n",
        numbers, d));
    Ok(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

// We cargo run again to compile everything
// By going to http://localhost:3000 we see the basic html layout we setup and now we can calculate the greatest common divisor.