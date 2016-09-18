#[macro_use]
extern crate nickel;
extern crate uuid;
extern crate bson;
extern crate mongodb;

use nickel::{Nickel, HttpRouter, MediaType};
use uuid::Uuid;
use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

fn main()
{
    let mut server = Nickel::new();

    server.get("/newUUID", middleware! { |_, mut response|
        let uuid = Uuid::new_v4();
        let mut json_string = "{\"uuid\": \"".to_owned();
        json_string.push_str(&uuid.to_string());
        json_string.push_str("\" }");

        let client = Client::conect("localhost", 27017).ok().except("Could not connect to users DB");
        let usersDb = client.db("admin").co

        response.set(MediaType::Json);
        json_string
    });

    server.listen("0.0.0.0:20000");
}
