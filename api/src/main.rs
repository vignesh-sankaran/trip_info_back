#[macro_use] extern crate nickel;
extern crate uuid;

use nickel::{Nickel, HttpRouter, MediaType};
use uuid::Uuid;

fn main()
{
    let mut server = Nickel::new();

    server.get("/newUUID", middleware! { |_, mut response|
        let uuid = Uuid::new_v4();
        let mut json_string = "{\"uuid\": \"".to_owned();
        json_string.push_str(&uuid.to_string());
        json_string.push_str("\" }");

        response.set(MediaType::Json);
        json_string
    });

    server.listen("0.0.0.0:20000");
}
