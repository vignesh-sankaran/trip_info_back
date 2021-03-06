extern crate iron;
extern crate router;
extern crate uuid;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate dotenv;
extern crate hyper_native_tls;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

pub mod schema;
pub mod models;

pub mod serde_types;
mod db_lib;

use std::io::Read;
use iron::prelude::*;
use iron::status;
use router::Router;
use uuid::Uuid;
use serde_types::*;
use db_lib::*;

pub fn start_server(ip: std::net::Ipv4Addr, port: u16) -> iron::Listening {
    let ssl_config = hyper_native_tls::NativeTlsServer::new("./ssl/identity.p12", "testpass")
        .unwrap();

    Iron::new(router_config()).https((ip, port), ssl_config).unwrap()
}

fn router_config() -> Router {
    let mut router: Router = Router::new();
    router.get("/newUUID", new_uuid_handler, "newUUID");
    router.post("/journey/home", journey_home_init, "home_post");
    router.post("/journey/destination",
                journey_destination_init,
                "destination_post");
    router
}

fn new_uuid_handler(_: &mut Request) -> IronResult<Response> {
    let uuid_string = Uuid::new_v4().to_string();
    let uuid_struct = UUID { uuid: uuid_string.clone() };
    let uuid_json = serde_json::to_string(&uuid_struct).unwrap();
    let connection = establish_connection();
    let _ = create_new_user(&connection, &uuid_string);
    Ok(Response::with((status::Ok, uuid_json)))
}

fn journey_home_init(req: &mut Request) -> IronResult<Response> {
    let connection = establish_connection();
    // Credit for post body processing to RenaudParis at https://github.com/iron/iron/issues/391. Retrieved 14 January 2016
    let mut post_body_raw = String::new();
    req.body.read_to_string(&mut post_body_raw).unwrap();
    let post_body_struct: HomeInfoAdd = serde_json::from_str(post_body_raw.trim()).unwrap(); // Need to define the struct prior to serde deserialise
    // This need to be handled to throw a 500 error if this fails
    // Also, dereferencing String goes to &str, there's a StackOverflow link somewhere...
    let _ = update_user_home(&connection,
                             &post_body_struct.uuid,
                             &post_body_struct.home_address_text,
                             &post_body_struct.home_address_lat,
                             &post_body_struct.home_address_long);
    Ok(Response::with((status::Ok)))
}

fn journey_destination_init(req: &mut Request) -> IronResult<Response> {
    let connection = establish_connection();
    let mut req_raw = String::new();
    req.body.read_to_string(&mut req_raw).unwrap();
    // Need to define the struct prior to serde deserialise
    let req_struct: DestinationInfoAdd = serde_json::from_str(req_raw.trim()).unwrap();
    let _ = update_user_destination(&connection,
                                    req_struct.uuid.trim(),
                                    req_struct.destination_address_text.trim(),
                                    req_struct.destination_address_lat.trim(),
                                    req_struct.destination_address_long.trim());
    Ok(Response::with((status::Ok)))
}
