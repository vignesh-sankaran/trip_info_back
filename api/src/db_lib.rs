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
    include!(concat!(env!("OUT_DIR"), "/db_lib.rs"));
    include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

    extern crate diesel;
    extern crate dotenv;
    extern crate serde;
    extern crate serde_json;

    use diesel::pg::PgConnection;

    // Helper DB connection method. 
    // Don't want to rely on an external function that is also being unit tested
    fn helper_db_connection() -> PgConnection
    {
        use std::env;
        use dotenv::dotenv;
        use diesel::prelude::*;

        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
    }

    // See if we can find a way to pass the DB connection in rather than creating a new one here
    fn helper_delete_user(conn: &PgConnection, uuid_string: &str)
    {
        use self::schema::user_info::dsl::{user_info, uuid};
        use diesel::*;
        
        let _ = diesel::delete(user_info.filter(uuid.like(format!("%{}%", uuid_string))))
            .execute(conn)
            .expect("Failed to delete records with old UUID");
    }

    fn helper_create_user(conn: &PgConnection, uuid_string: &str)
    {
        use self::schema::user_info;
        use diesel::insert;
        use diesel::prelude::*;
        use self::models::{UserInfo, NewUser};

        let new_user = NewUser
        {
            uuid: uuid_string,
        };

        let _: UserInfo = insert(&new_user).into(user_info::table)
            .get_result(conn)
            .expect("Error inserting new user into user_info DB");
    }
   
    #[test]
    fn test_db_connection()
    {
        // Find a way to test if the result of the connection is true or false
        assert!(true);
    }

    #[test]
    fn test_create_new_user()
    {
        use self::schema::user_info::dsl::{user_info, uuid};
        use diesel::*;
        
        let uuid_string = "090ea3e2-5f2e-4c9a-9a83-c23f27d959a2";
        let db_conn = helper_db_connection();

        // If the UUID string already exists, delete all records with it

        helper_delete_user(&db_conn, uuid_string);

        let _ = super::create_new_user(&db_conn, uuid_string);

        // Get the last row, check the uuid of that last row against what we have here
        let new_record = user_info.filter(uuid.eq(uuid_string))
            .load::<self::models::UserInfo>(&db_conn)
            .expect("Couldn't load up the db");
                        
        assert!(new_record.last().unwrap().uuid == uuid_string);
    }

    #[test]
    fn test_update_user_home()
    {
        use self::schema::user_info::dsl::{user_info, uuid}; 
        use diesel::*;

        let uuid_string = "64b167fe-9069-4b1a-be2d-b20cfd87b263";
        let db_conn = helper_db_connection();

        helper_delete_user(&db_conn, uuid_string);
        helper_create_user(&db_conn, uuid_string);

        let db_conn = helper_db_connection();
        let home_address_text_string = "100 Bogong Avenue, Glen Waverley VIC 3150";
        let home_address_lat_string = "-37";
        let home_address_long_string = "142";

        let _ = super::update_user_home(&db_conn, uuid_string, &home_address_text_string, &home_address_lat_string, &home_address_long_string);

        let last_entry_raw = user_info.filter(uuid.eq(uuid_string))
            .load::<self::models::UserInfo>(&db_conn)
            .expect("Couldn't load up the db");

        let last_entry = last_entry_raw.last().unwrap();

        assert!(last_entry.home_address_text == home_address_text_string);
        assert!(last_entry.home_address_lat == home_address_lat_string);
        assert!(last_entry.home_address_long == home_address_long_string);
    }

    #[test]
    fn test_update_user_destination()
    {
        let _ = "87265ef6-cf83-4e66-8f85-fc54fbb38de9";
        assert!(true);
    }
}