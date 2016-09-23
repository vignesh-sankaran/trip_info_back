#[macro_use]
extern crate nickel;
extern crate uuid;
#[macro_use(bson, doc)]
extern crate mongodb;

use nickel::{Nickel, HttpRouter, MediaType};
use uuid::Uuid;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

fn main()
{
    let mut server = Nickel::new();

    server.get("/newUUID", middleware! { |_, mut response|
        let uuid = Uuid::new_v4();
        let uuid_string = &uuid.to_string();
        let mut json_string = "{\"uuid\": \"".to_owned();
        json_string.push_str(uuid_string);
        json_string.push_str("\" }");

        let client = Client::with_uri("mongodb://db:27017").unwrap();

        let users_coll = client.db("tripinfo").collection("users");
        let id_entry = doc!{"id" => uuid_string};
        users_coll.insert_one(id_entry, None).ok().expect("Failed to insert new user ID into users DB");

        response.set(MediaType::Json);
        json_string
    });

    server.listen("0.0.0.0:20000");
}
