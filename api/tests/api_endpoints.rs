extern crate hyper;
extern crate hyper_native_tls;
extern crate openssl;
extern crate diesel;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

#[test]
fn test_new_uuid()
{    
    let mut url = hyper::Url::parse("https://127.0.0.1/newUUID").unwrap();
    let _ = url.set_port(Some(20000));

    // Create OpenSSL context to disable certificate verification
    let openssl_settings = openssl::ssl::SslContextBuilder::new(openssl::ssl::SslMethod::tls());

    let tls = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(tls);
    let client = Client::with_connector(connector);

    let response = client.get(url).send();

    assert!(response.unwrap().status == hyper::status::StatusCode::Ok);
    // Re enable certificate verification
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