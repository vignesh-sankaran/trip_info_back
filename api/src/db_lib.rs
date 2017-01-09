//#![feature(proc_macro)]
include!(concat!(env!("OUT_DIR"), "/db_lib.rs"));

use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

use self::models::{UserInfo, NewUser};

pub fn establish_connection() -> PgConnection
{
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}