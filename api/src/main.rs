#[macro_use]
extern crate nickel;
extern crate uuid;
#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
extern crate rustc_serialize;

use nickel::{Nickel, JsonBody, HttpRouter, MediaType};
use nickel::status::StatusCode::{self};
use rustc_serialize::json::{self,Json, ToJson};
use uuid::Uuid;
use bson::Bson;
use mongodb::{Client, ThreadedClient, CommandType};
use mongodb::error::Error::OperationError;
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

        let client = Client::with_uri("mongodb://localhost:27017").unwrap();

        let auth_db = client.db("auth");
        match auth_db.auth("system", "system")
        {
            Err(OperationError(_)) => (),
            Err(_) => panic!("Expected OperationError for invalid authentication, but got some other error instead"),
            _ => panic!("Authentication succeeded despite invalid credentials")
        };

        let users_coll = client.db("tripinfo").collection("users");
        let id_entry = doc!{"id" => uuid_string};
        users_coll.insert_one(id_entry, None).ok().expect("Failed to insert new user ID into users DB");

        response.set(MediaType::Json);
        json_string
    });

    server.listen("0.0.0.0:20000");
}
