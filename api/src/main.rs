extern crate iron;
extern crate uuid;
extern crate router;
extern crate serde;
extern crate serde_json;

use std::path::PathBuf;
use iron::prelude::*;
use iron::status;
use router::Router;
use uuid::Uuid;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

fn main()
{
    let mut router: Router = Router::new();
    router.get("/newUUID", new_uuid_handler, "newUUID");

    Iron::new(router).https("0.0.0.0:20000", PathBuf::from("./ssl/cert.pem"), PathBuf::from("./ssl/dec.pem")).unwrap();
}

fn new_uuid_handler(_: &mut Request) -> IronResult<Response>
{
    let uuid_struct = UUID { uuid: Uuid::new_v4().to_string() };
    let uuid_json = serde_json::to_string(&uuid_struct).unwrap();
    Ok(Response::with((status::Ok, uuid_json)))
}
