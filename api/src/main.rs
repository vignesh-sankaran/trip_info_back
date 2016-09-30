extern crate iron;
extern crate uuid;
extern crate router;

use std::path::PathBuf;
use iron::prelude::*;
use iron::status;
use router::Router;
use uuid::Uuid;


fn main()
{
    let mut router: Router = Router::new();
    router.get("/newUUID", new_uuid_handler, "newUUID");

    Iron::new(router).https("0.0.0.0:20000", PathBuf::from("./ssl/cert.pem"), PathBuf::from("./ssl/dec.pem")).unwrap();
}

fn new_uuid_handler(_: &mut Request) -> IronResult<Response>
{
    let uuid = Uuid::new_v4();
    let uuid_string = &uuid.to_string();
    let mut json_string = "{\"uuid\": \"".to_owned();
    json_string.push_str(uuid_string);
    json_string.push_str("\" }");
    Ok(Response::with((status::Ok)))
}
