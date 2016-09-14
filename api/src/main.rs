#[macro_use] extern crate nickel;
extern crate uuid;

use nickel::{Nickel, HttpRouter, MediaType};
use uuid::Uuid;

fn main()
{
    let mut server = Nickel::new();
/*
    server.get("/newUUID", middleware! { |request, mut response|
        let uuid = Uuid::new_v4();
        let mut json_string = "{\"uuid\": ".to_owned();
        json_string.push_str(&uuid.to_string());
        json_string.push_str(" }");

        response.set(MediaType::Json);
        response.send(json_string);

    });
    */
    server.utilize(router! {
        get "**" => |_req, _res| {
            "Hello world!"
        }
    });
    server.listen("127.0.0.1:10000");
}
