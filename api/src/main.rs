extern crate iron;
extern crate router;
extern crate uuid;
extern crate serde;
extern crate serde_json;
extern crate dotenv;
extern crate params;
#[macro_use] extern crate diesel;

mod db_lib;

use std::path::PathBuf;
use iron::prelude::*;
use iron::status;
use router::Router;
use uuid::Uuid;
use db_lib::*;
use std::io::Read;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

fn main()
{
    let mut router: Router = Router::new();
    router.get("/newUUID", new_uuid_handler, "newUUID");
    router.post("/journey/home", journey_home_init, "home_post");

    Iron::new(router).https("0.0.0.0:20000", PathBuf::from("./ssl/cert.pem"), PathBuf::from("./ssl/dec.pem")).unwrap();
}

fn new_uuid_handler(_: &mut Request) -> IronResult<Response>
{
    let uuid_string = Uuid::new_v4().to_string();
    let uuid_struct = UUID { uuid: uuid_string.clone() };
    let uuid_json = serde_json::to_string(&uuid_struct).unwrap();
    let connection = establish_connection();
    let _ = create_new_user(&connection, &uuid_string);
    Ok(Response::with((status::Ok, uuid_json)))
}

fn journey_home_init(req: &mut Request) -> IronResult<Response>
{
    let connection = establish_connection();
    let mut post_body_raw = String::new();
    req.body.read_to_string(&mut post_body_raw).unwrap();
    let post_body_struct: HomeInfoAdd = serde_json::from_str(post_body_raw.trim()).unwrap();
    let _ = update_user_home(&connection, post_body_struct.uuid.trim(), post_body_struct.home_address_text.trim(), 
                                post_body_struct.home_address_lat.trim(), post_body_struct.home_address_long.trim());
    Ok(Response::with((status::Ok)))
}