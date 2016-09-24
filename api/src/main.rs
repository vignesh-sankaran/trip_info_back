#[macro_use]
extern crate nickel;
extern crate uuid;
#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

use nickel::{Nickel, HttpRouter, MediaType};
use uuid::Uuid;
use mongodb::{Client, ThreadedClient, CommandType};
use mongodb::db::ThreadedDatabase;
use bson::Bson;


fn main()
{
    let mut server = Nickel::new();

    server.get("/newUUID", middleware! { |_, mut response|
        let uuid = Uuid::new_v4();
        let uuid_string = &uuid.to_string();
        let mut json_string = "{\"uuid\": \"".to_owned();
        json_string.push_str(uuid_string);
        json_string.push_str("\" }");

        let doc = doc! { "connectionStatus" => 1};
        let client = Client::with_uri("mongodb://db:27017").unwrap();
        let auth_db = client.db("auth");

        let before = auth_db.command(doc.clone(), CommandType::Suppressed, None).unwrap();

        let info = match before.get("authInfo") {
            Some(&Bson::Document(ref doc)) => doc.clone(),
            _ => panic!("Invalid response for initial connectionStatus command")
        };

        match info.get("authenticatedUsers") {
            Some(&Bson::Array(ref vec)) => assert!(!vec.is_empty()),
            _ => panic!("This list doens't have any users in it...")
        };

        auth_db.auth("system", "system").unwrap();
        
        let users_coll = client.db("tripinfo").collection("users");
        let id_entry = doc!{"id" => uuid_string};
        users_coll.insert_one(id_entry, None).ok().expect("Failed to insert new user ID into users DB");

        response.set(MediaType::Json);
        json_string
    });

    server.listen("0.0.0.0:20000");
}
