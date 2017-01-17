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

pub fn update_user_destination<'a>(conn: &PgConnection, a_uuid: &'a str, a_destination_address_text: &'a str,
                                    a_destination_address_lat: &'a str, a_destination_address_long: &'a str) -> UserInfo
{
    use self::schema::user_info::dsl::{user_info, destination_address_text, destination_address_lat, destination_address_long};

    update(user_info.find(a_uuid))
        .set((destination_address_text.eq(a_destination_address_text),
            destination_address_lat.eq(a_destination_address_lat),
            destination_address_long.eq(a_destination_address_long)))
        .get_result::<UserInfo>(conn)
        .expect(&format!("Unable to update destination details for uuid {}", a_uuid))
}

/*
    Note: When the next Rust version comes out, consider putting mocking for the 
    database methods in here
*/
#[cfg(test)]
mod test
{
    extern crate diesel;
    extern crate dotenv;
    extern crate serde;
    extern crate serde_json;

    use diesel::pg::PgConnection;

    include!(concat!(env!("OUT_DIR"), "/db_lib.rs"));

    // Helper DB connection method. 
    // Don't want to rely on an external function that is also being unit tested
    fn connection() -> PgConnection
    {
        use std::env;
        use dotenv::dotenv;
        use diesel::prelude::*;
        use diesel::pg::PgConnection;

        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
    }

    #[test]
    fn test_db_connection()
    {
        assert!(true);
    }

    #[test]
    fn test_create_new_user()
    {
        let db_conn = connection();
        let uuid_string = "87265ef6-cf83-4e66-8f85-fc54fbb38de9";
        
        let result = super::create_new_user(&db_conn, &uuid_string);
        // Get the last row, check the uuid of that last row against what we have here

        use self::schema::user_info::dsl::{user_info, uuid};
        use diesel::*;

        let result = user_info.filter(uuid.eq("87265ef6-cf83-4e66-8f85-fc54fbb38de9"))
            .limit(1)
            .load::<self::models::UserInfo>(&db_conn)
            .expect("Couldn't load up the db");
                
        assert!(result.last().unwrap().uuid == uuid_string);
    }

    #[test]
    fn test_update_user_home()
    {
        assert!(true);
    }

    #[test]
    fn test_update_user_destination()
    {
        assert!(true);
    }
}