extern crate hyper;
extern crate diesel;
extern crate serde;
extern crate serde_json;
extern crate trip_info_api_lib;

mod api_endpoints_helper;

use std::io::Read;
use trip_info_api_lib::*;
use trip_info_api_lib::models;
use trip_info_api_lib::schema;
use trip_info_api_lib::serde_types::*;
use diesel::*;
use self::schema::user_info::dsl::{user_info, uuid};

#[test]
fn test_new_uuid() {
    use api_endpoints_helper::{db_connection, helper_delete_user, create_client};

    let port = 20000;
    let mut url = hyper::Url::parse("https://127.0.0.1/newUUID").unwrap();
    let _ = url.set_port(Some(port));
    let client = create_client();
    let mut server = start_server(std::net::Ipv4Addr::new(0, 0, 0, 0), port);

    let mut response = client.get(url).send().unwrap();
    assert_eq!(response.status, hyper::status::StatusCode::Ok);

    let mut response_string = String::new();
    {
        response.read_to_string(&mut response_string).unwrap();
    }
    let response_body_json: UUID = serde_json::from_str(&response_string).unwrap();
    let response_uuid = response_body_json.uuid;

    let db_conn = db_connection();
    let last_entry_raw = user_info.filter(uuid.eq(response_uuid.clone()))
        .load::<self::models::UserInfo>(&db_conn)
        .expect("Couldn't load up the db");

    let last_entry = last_entry_raw.last().unwrap();

    assert_eq!(response_uuid, last_entry.uuid);
    helper_delete_user(&db_conn, &response_uuid);

    let _ = server.close();
}

#[test]
fn test_add_user_home() {
    use api_endpoints_helper::*;

    let port = 20001;
    let mut url = hyper::Url::parse("https://127.0.0.1/journey/home").unwrap();
    let _ = url.set_port(Some(port));
    let client = create_client();
    let mut server = start_server(std::net::Ipv4Addr::new(0, 0, 0, 0), port);

    let uuid_string = "7613f9cf-a43d-4807-a03f-080ade906236";
    let db_conn = db_connection();

    let home_address_text_string = "100 Bogong Avenue, Glen Waverley VIC 3150";
    let home_address_lat_string = "-37";
    let home_address_long_string = "142";

    create_user(&db_conn, &uuid_string);

    // Create the POST body
    let request_body_struct = HomeInfoAdd {
        uuid: uuid_string.to_string(),
        home_address_text: home_address_text_string.to_string(),
        home_address_lat: home_address_lat_string.to_string(),
        home_address_long: home_address_long_string.to_string(),
    };

    let request_body_json = serde_json::to_string(&request_body_struct).unwrap();
    let response = client.post(url).body(&request_body_json).send().unwrap();

    assert_eq!(response.status, hyper::status::StatusCode::Ok);

    let last_entry_raw = user_info.filter(uuid.eq(uuid_string.clone()))
        .load::<self::models::UserInfo>(&db_conn)
        .expect("Couldn't load up the db");

    let last_entry = last_entry_raw.last().unwrap();
    assert!(last_entry.home_address_text == home_address_text_string);
    assert!(last_entry.home_address_lat == home_address_lat_string);
    assert!(last_entry.home_address_long == home_address_long_string);

    helper_delete_user(&db_conn, &uuid_string);

    let _ = server.close();
}

#[test]
fn test_add_user_destination() {
    use api_endpoints_helper::*;
    let port = 20002;
    let mut url = hyper::Url::parse("https://127.0.0.1/journey/destination").unwrap();
    let _ = url.set_port(Some(port));
    let uuid_string = "3d683ffe-c03e-49a6-8662-c0d69decaeee";
    let client = create_client();
    let mut server = start_server(std::net::Ipv4Addr::new(0, 0, 0, 0), port);

    assert!(true);
    let _ = server.close();
}
