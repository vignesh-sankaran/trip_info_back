extern crate hyper;
extern crate hyper_openssl;
extern crate openssl;
extern crate diesel;

use hyper::Client;
use hyper::net::HttpsConnector;
use openssl::ssl::*;

#[test]
fn test_new_uuid()
{    
    let mut url = hyper::Url::parse("https://127.0.0.1/newUUID").unwrap();
    let _ = url.set_port(Some(20000));

    // Use pkcs12 module in openssl to parse it and pass into ssl context builder
    let mut ssl_connector_builder = SslConnectorBuilder::new(SslMethod::tls()).unwrap();
    {
        let mut ssl_context_builder = ssl_connector_builder.builder_mut();
        ssl_context_builder.set_verify(SSL_VERIFY_NONE);
        let _ = ssl_context_builder.set_ca_file("./ssl/cert.pem").unwrap();
        let _ = ssl_context_builder.set_private_key_file("./ssl/dec.pem", openssl::x509::X509_FILETYPE_PEM).unwrap();
    }
    let ssl_connector = ssl_connector_builder.build();
    let mut openssl_client = hyper_openssl::OpensslClient::from(ssl_connector);
    openssl_client.danger_disable_hostname_verification(true);
    let connector = HttpsConnector::new(openssl_client);
    let client = Client::with_connector(connector);

    let response = client.get(url).send();

    assert!(response.unwrap().status == hyper::status::StatusCode::Ok);
}

#[test]
fn test_add_user_home()
{
    assert!(true);
}

#[test]
fn test_add_user_destination()
{
    assert!(true);
}