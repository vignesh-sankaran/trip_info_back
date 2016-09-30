extern crate iron;
extern crate uuid;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;
use uuid::Uuid;


fn main()
{
    let mut router: Router = Router::new();
    router.get("/newUUID", newUUID_handler, "newUUID");

    Iron::new(router).http("localhost:20000").unwrap();
}

fn newUUID_handler(req: &mut Request) -> IronResult<Response>
{
    let uuid = Uuid::new_v4();
    let uuid_string = &uuid.to_string();
    let mut json_string = "{\"uuid\": \"".to_owned();
    json_string.push_str(uuid_string);
    json_string.push_str("\" }");
    Ok(Response::with((status::Ok)))
}
