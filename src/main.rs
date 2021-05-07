// extern allows the iron and mime crates that we cited in our Cargo.toml file available to our program
extern crate iron;
// The macro_use attribute alerts Rust that we plan to use macros exported by this crate.
#[macro_use] extern crate mime;

// The declaration below makes all the public names of the iron::prelude module directly visible in our code.
use iron::prelude::*;
// Generally it's preferable to spell out the name we wish to use as below, but by convention, when a module is named prelude, that means that its exports are intended to provide the sort of general facilities that any user of the crate will probably need. So in this case, using a wildcard * makes sense.
use iron::status;

fn main() {
    // Printing a msg reminding us how to connect to our server
    println!("Serving on http://localhost:3000...");
    // Calling Iron::new to create a server, and then sets it listening on TCP port 3000
    // We pass the get_form function to Iron::new, indicating that the server should use that function to handle all requests.
    Iron::new(get_form).http("localhost:3000").unwrap();
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
    response.set_mut(r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="n">
            <input type="text" name="n">
            <button type="submit">Compute GCD</button>
        </form>
    "#);

    // Our function's return type, IronResult<Response> (at the top of function), is another variant of the Result type we encountered earlier (Ok(r) or Err(e)). We construct our return value as Ok(response) using the "last expression" syntax to implicitly specify the function's return value (of semi-colon)
    Ok(response)

    // We use cargo run which will fetch the needed crates, compile them, building our program, linking everything, and starting it up.
    // If everything is ok, we can go to http://localhost:3000/gcd
}
