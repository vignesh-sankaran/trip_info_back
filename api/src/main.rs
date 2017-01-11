extern crate iron;
extern crate uuid;
extern crate router;
extern crate serde;
extern crate serde_json;
extern crate dotenv;
#[macro_use] extern crate diesel;

mod db_lib;

use std::path::PathBuf;
use iron::prelude::*;
use iron::status;
use router::Router;
use uuid::Uuid;
use db_lib::*;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

fn main()
{
    let mut router: Router = Router::new();
    router.get("/newUUID", new_uuid_handler, "newUUID");

    Iron::new(router).https("0.0.0.0:20000", PathBuf::from("./ssl/cert.pem"), PathBuf::from("./ssl/dec.pem")).unwrap();
}

fn new_uuid_handler(_: &mut Request) -> IronResult<Response>
{
    let uuid_string = Uuid::new_v4().to_string();
    let uuid_struct = UUID { uuid: uuid_string.clone() };
    let uuid_json = serde_json::to_string(&uuid_struct).unwrap();
    let connection = establish_connection();
    let result = create_new_user(&connection, &uuid_string);
    Ok(Response::with((status::Ok, uuid_json)))
}