extern crate openssl;
extern crate serde_codegen;
extern crate runas;
extern crate time;

use std::env;
use std::path::Path;
use std::fs::*;
use std::process;
use std::io::Read;

fn main() {
    extern crate diesel_codegen_syntex;

    if check_cert_expiry_macos() {
        setup_local_ssl_macos();
    }

    // Codegen for Diesel
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let src_diesel = Path::new("src/db_lib.in.rs");
    let dst_diesel = Path::new(&out_dir).join("db_lib.rs");

    diesel_codegen_syntex::expand(&src_diesel, &dst_diesel).unwrap();
}

// Return true if the certificate has expired
fn check_cert_expiry_macos() -> bool {
    // Read certs
    if Path::new("./ssl").exists() {
        let mut identity_file = std::fs::File::open("./ssl/identity.p12").unwrap();
        let mut pkcs12 = vec![];
        identity_file.read_to_end(&mut pkcs12).unwrap();
        let pkcs12 = openssl::pkcs12::Pkcs12::from_der(&pkcs12).unwrap();
        let identity = pkcs12.parse("testpass").unwrap();

        let not_after_string = identity.cert.not_after().to_string();
        // Reference for date time parsing: http://www.cplusplus.com/reference/ctime/strftime/
        let not_after_time = time::strptime(&not_after_string, "%b %d %H:%M:%S %Y %Z").unwrap();

        // Looks like operator overloading can be done in Rust too :)
        if time::now() < not_after_time {
            return false;
        }
    }
    return true;
}

fn setup_local_ssl_macos() {
    // Unlock the System keychain
    process::Command::new("security")
        .arg("unlock-keychain")
        .arg("-u")
        .arg("/Library/Keychains/System.keychain")
        .output()
        .expect("Failed to unlock login keychain");

    // Delete all certs inside ssl directory if it exists
    if Path::new("./ssl").exists() {
        runas::Command::new("security")
            .arg("delete-certificate")
            .arg("-c")
            .arg("localhost")
            .status()
            .expect("Unable to delete certificate");

        remove_dir_all(Path::new("./ssl")).expect("Unable to delete ssl directory");
    }

    create_dir("./ssl/").expect("Unable to create new ssl directory");

    // Create new ssl key and certificate
    // Todo: Use the rust-openssl bindings to do this instead
    process::Command::new("openssl")
        .current_dir("./ssl/")
        .arg("req")
        .arg("-x509")
        .arg("-newkey")
        .arg("rsa:4096")
        .arg("-keyout")
        .arg("key.pem")
        .arg("-out")
        .arg("cert.pem")
        .arg("-days")
        .arg("60")
        .arg("-nodes")
        .arg("-subj")
        .arg("/C=AU/ST=Victoria/L=Melbourne/O=Ferndrop Pty Ltd/OU=org/CN=localhost")
        .output()
        .expect("Failed to create ssl key and certificate");

    // Combine certificate and private key into one identity
    // Todo: Use the rust-openssl bindings to do this too
    process::Command::new("openssl")
        .current_dir("./ssl/")
        .arg("pkcs12")
        .arg("-export")
        .arg("-out")
        .arg("identity.p12")
        .arg("-inkey")
        .arg("key.pem")
        .arg("-in")
        .arg("cert.pem")
        .arg("-passout")
        .arg("pass:testpass")
        .output()
        .expect("Failed to create a new identity");

    // Decrypt the ssl key
    process::Command::new("openssl")
        .arg("rsa")
        .arg("-in")
        .arg("./ssl/key.pem")
        .arg("-out")
        .arg("./ssl/dec.pem")
        .output()
        .expect("Failed to decrypt the private key");

    // Change the decrypted private key to be readonly
    let mut permissions = metadata("./ssl/dec.pem").unwrap().permissions(); // Is there a safer way to do this?
    permissions.set_readonly(true);
    let result = set_permissions("./ssl/dec.pem", permissions);

    // http://stackoverflow.com/questions/30320083/how-to-print-a-vec, credit to Matthieu M, retrieved on 28 Dec 2016
    println!("Change permission of decrypted private key: {:?}", result);

    // Add the certificate into the System keychain
    // If we have some time to kill, make a Rust to macOS Keychain bindings library
    runas::Command::new("security")
        .arg("add-trusted-cert")
        .arg("-d")
        .arg("-r")
        .arg("trustRoot")
        .arg("-k")
        .arg("/Library/Keychains/System.keychain")
        .arg("./ssl/cert.pem")
        .status()
        .expect("Failed to add cert to keychain");

    runas::Command::new("security")
        .arg("import")
        .arg("./ssl/identity.p12")
        .arg("-t")
        .arg("agg")
        .arg("-k")
        .arg("/Library/Keychains/System.keychain")
        .arg("-P")
        .arg("testpass")
        .status()
        .expect("Failed to add identity to keychain");

    // Lock the System keychain once certificate has been inserted
    process::Command::new("security")
        .arg("lock-keychain")
        .arg("/Library/Keychains/System.keychain")
        .output()
        .expect("Failed to lock System keychain");
}
