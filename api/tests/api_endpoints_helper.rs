extern crate diesel;
extern crate dotenv;
extern crate trip_info_api_lib;
extern crate hyper;
extern crate hyper_openssl;
extern crate openssl;

use diesel::pg::PgConnection;
use self::hyper::Client;


pub fn helper_db_connection() -> PgConnection {
    use std::env;
    use dotenv::dotenv;
    use diesel::prelude::*;

    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

pub fn helper_delete_user(conn: &PgConnection, uuid_string: &str) {
    use trip_info_api_lib::schema::user_info::dsl::{user_info, uuid};
    use diesel::*;

    let _ = diesel::delete(user_info.filter(uuid.like(format!("%{}%", uuid_string))))
        .execute(conn)
        .expect("Failed to delete records with old UUID");
}

// Create hyper client that works with self-signed certs
pub fn create_client() -> Client {
    // Apparently using this in one line throws a hissy fit, not quite sure why
    use std::fs::*;
    use std::io::Read;
    use self::hyper::net::HttpsConnector;
    use self::openssl::ssl::*;

    let mut identity_file = File::open("./ssl/identity.p12").unwrap();
    let mut pkcs12 = vec![];
    identity_file.read_to_end(&mut pkcs12).unwrap();
    let pkcs12 = openssl::pkcs12::Pkcs12::from_der(&pkcs12).unwrap();
    let identity = pkcs12.parse("testpass").unwrap();

    let mut ssl_connector_builder = SslConnectorBuilder::new(SslMethod::tls()).unwrap();
    {
        let mut ssl_context_builder = ssl_connector_builder.builder_mut();
        ssl_context_builder.set_verify(SSL_VERIFY_NONE);
        let result = ssl_context_builder.set_certificate(&identity.cert);
        let _ = ssl_context_builder.set_private_key(&identity.pkey);

        match result {
            Ok(_) => println!("This worked!"),
            Err(result) => panic!("{}", result.errors().first().unwrap().reason().unwrap()),
        }
    }
    let ssl_connector = ssl_connector_builder.build();
    let mut openssl_client = hyper_openssl::OpensslClient::from(ssl_connector);
    {
        openssl_client.danger_disable_hostname_verification(true);
    }
    let connector = HttpsConnector::new(openssl_client);
    let client = Client::with_connector(connector);
    client
}
