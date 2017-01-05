#![cfg_attr(feature = "dev-local-nightly", feature(proc_macro))]

#[cfg(feature = "dev-local-nightly")]
include!("db_lib.in.rs");

#[cfg(feature = "dev-local-stable")]
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
    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url));
}