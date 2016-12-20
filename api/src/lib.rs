#![feature(proc_macro)]

#[macro_use] extern crate diesel_codegen;

pub mod schema;
pub mod models;

#[cfg(feature = "default")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

#[macro_use] extern crate diesel;
extern crate dotenv;

use disel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection
{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
}