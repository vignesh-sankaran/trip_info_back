include!(concat!(env!("OUT_DIR"), "/db_lib.rs"));
use std::env;
use diesel::insert;
use diesel::update;
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

pub fn create_new_user<'a>(conn: &PgConnection, uuid: &'a str) -> UserInfo
{
    use self::schema::user_info;

    let new_user = NewUser
    {
        uuid: uuid,
    };

    insert(&new_user).into(user_info::table)
        .get_result(conn)
        .expect("Error inserting new user into user_info DB")
}

pub fn update_user_home<'a>(conn: &PgConnection, a_uuid: &'a str, a_home_address_text: &'a str,
                            a_home_address_lat: &'a str, a_home_address_long: &'a str) -> UserInfo
{
    use self::schema::user_info::dsl::{user_info, home_address_text, home_address_lat, home_address_long};
    
    update(user_info.find(a_uuid))
        // Need to wrap multiple value insert in tuple
        .set((home_address_text.eq(a_home_address_text),
            home_address_lat.eq(a_home_address_lat),
            home_address_long.eq(a_home_address_long))) 
        .get_result::<UserInfo>(conn)
        .expect(&format!("Unable to update home details for uuid {}", a_uuid))
}