extern crate hyper;
extern crate hyper_openssl;
extern crate openssl;
extern crate diesel;
extern crate serde;
extern crate serde_json;

use std::io::Read;
use hyper::Client;
use hyper::net::HttpsConnector;
use openssl::ssl::*;

#[test]
fn test_new_uuid() {
    let mut url = hyper::Url::parse("https://127.0.0.1/newUUID").unwrap();
    let _ = url.set_port(Some(20000));

    let mut identity_file = std::fs::File::open("./ssl/identity.p12").unwrap();
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

    let response = client.get(url).send().unwrap();
    assert!(response.status == hyper::status::StatusCode::Ok);

    //    let response_string = response.read_to_string();
    //    let response_body_json: UUID = serde_json::from_str(response_string).unwrap();
    //    let response_uuid = response_body_json.uuid;
}

#[test]
fn test_add_user_home() {
    assert!(true);
}

#[test]
fn test_add_user_destination() {
    assert!(true);
}
